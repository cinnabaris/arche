use std::io::{Cursor, Read};
use std::sync::Arc;

use futures::{Future, Stream};
use hyper::{self, header::CONTENT_TYPE, Body, Method, Request, Response, StatusCode};
use juniper::{graphiql::graphiql_source, http, FieldError, GraphQLType, InputValue, RootNode};
use mime;
use serde_json;

use super::super::{context::Context, errors::Result, router::Route};

pub struct Doc {}
impl Route for Doc {
    fn handle(&self, _: Arc<Context>, _: &Request<Body>) -> Result<(mime::Mime, Response<Body>)> {
        let mut res = Response::new(Body::empty());
        *res.body_mut() = Body::from(graphiql_source("/graphql"));
        Ok((mime::TEXT_HTML_UTF_8, res))
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

#[derive(Debug, Deserialize, PartialEq)]
pub struct GraphQLRequest(GraphQLBatchRequest);

pub struct GraphQLResponse(StatusCode, String);

impl GraphQLRequest {
    // pub fn new(req: &Request<Body>) -> Result<Option<Self>> {
    //     if let Some(ref ct) = req.headers().get(CONTENT_TYPE) {};
    //
    //
    //     let it: Self = serde_json::from_slice(&body)?;
    //     Ok(Some(it))
    //     // req.body().concat().and_then(|body| {
    //     //     let it: Self = serde_json::from_slice(&body)?;
    //     //     Ok(Some(it))
    //     //     // let payload = deserialize(&body);
    //     //     // // you should make `db.insert` async and return a future too
    //     //     // db.insert(table, payload).then(|result| {
    //     //     //     match result {
    //     //     //         // ...
    //     //     //     }
    //     //     // })
    //     // })
    // }

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
//
// impl GraphQLResponse {
//     pub fn error(error: FieldError) -> Self {
//         let response = http::GraphQLResponse::error(error);
//         let json = serde_json::to_string(&response).unwrap();
//         GraphQLResponse(StatusCode::BAD_REQUEST, json)
//     }
//
//     pub fn custom(status: StatusCode, response: serde_json::Value) -> Self {
//         let json = serde_json::to_string(&response).unwrap();
//         GraphQLResponse(status, json)
//     }
// }
//
// impl FromData for GraphQLRequest {
//     type Error = String;
//

// }
//
// impl<'r> Responder<'r> for GraphQLResponse {
//     fn respond_to(self, _: &Request) -> Result<Response<'r>, Status> {
//         let GraphQLResponse(status, body) = self;
//
//         Ok(Response::build()
//             .header(ContentType::new("application", "json"))
//             .status(status)
//             .sized_body(Cursor::new(body))
//             .finalize())
//     }
// }
