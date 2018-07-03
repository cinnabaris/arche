use std::sync::Arc;

use http::request::Parts;
use hyper::StatusCode;
use juniper::{graphiql::graphiql_source, http, GraphQLType, RootNode};
use mime;
use serde_json;

use super::super::{context::Context, errors::Result, request, router::Route};

pub struct Doc {}
impl Route for Doc {
    fn handle(
        &self,
        _: Arc<Context>,
        _: &Parts,
        _: &Vec<u8>,
    ) -> Result<(StatusCode, mime::Mime, Option<String>)> {
        let buf = graphiql_source("/graphql");
        Ok((StatusCode::OK, mime::TEXT_HTML_UTF_8, Some(buf)))
    }
}

pub struct GraphQL {}
impl Route for GraphQL {
    fn handle(
        &self,
        ctx: Arc<Context>,
        parts: &Parts,
        body: &Vec<u8>,
    ) -> Result<(StatusCode, mime::Mime, Option<String>)> {
        let req: GraphQLBatchRequest = serde_json::from_slice(body)?;
        debug!("graphql query:\n{:?}", req);
        let req = GraphQLRequest(req);

        debug!("{:?}", parts);

        let ctx = super::context::Context {
            locale: request::locale(&parts).unwrap_or("en-US".to_string()),
            client_ip: request::client_ip(&parts).unwrap_or("::1".to_string()),
            token: request::token(&parts),
            state: ctx,
        };
        let sch = super::schema::Schema::new(super::query::Query, super::mutation::Mutation);
        let GraphQLResponse(code, buf) = req.execute(&sch, &ctx)?;
        Ok((code, mime::APPLICATION_JSON, Some(buf)))
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
enum GraphQLBatchRequest {
    Single(http::GraphQLRequest),
    Batch(Vec<http::GraphQLRequest>),
}

#[derive(Serialize)]
#[serde(untagged)]
enum GraphQLBatchResponse<'a> {
    Single(http::GraphQLResponse<'a>),
    Batch(Vec<http::GraphQLResponse<'a>>),
}

impl GraphQLBatchRequest {
    pub fn execute<'a, CtxT, QueryT, MutationT>(
        &'a self,
        root_node: &RootNode<QueryT, MutationT>,
        context: &CtxT,
    ) -> GraphQLBatchResponse<'a>
    where
        QueryT: GraphQLType<Context = CtxT>,
        MutationT: GraphQLType<Context = CtxT>,
    {
        match self {
            &GraphQLBatchRequest::Single(ref request) => {
                GraphQLBatchResponse::Single(request.execute(root_node, context))
            }
            &GraphQLBatchRequest::Batch(ref requests) => GraphQLBatchResponse::Batch(
                requests
                    .iter()
                    .map(|request| request.execute(root_node, context))
                    .collect(),
            ),
        }
    }
}

impl<'a> GraphQLBatchResponse<'a> {
    fn is_ok(&self) -> bool {
        match self {
            &GraphQLBatchResponse::Single(ref response) => response.is_ok(),
            &GraphQLBatchResponse::Batch(ref responses) => responses
                .iter()
                .fold(true, |ok, response| ok && response.is_ok()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GraphQLRequest(GraphQLBatchRequest);

pub struct GraphQLResponse(StatusCode, String);

impl GraphQLRequest {
    pub fn execute<CtxT, QueryT, MutationT>(
        &self,
        root_node: &RootNode<QueryT, MutationT>,
        context: &CtxT,
    ) -> Result<GraphQLResponse>
    where
        QueryT: GraphQLType<Context = CtxT>,
        MutationT: GraphQLType<Context = CtxT>,
    {
        let response = self.0.execute(root_node, context);
        let status = if response.is_ok() {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        };
        let json = serde_json::to_string(&response)?;

        Ok(GraphQLResponse(status, json))
    }
}
