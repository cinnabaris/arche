use juniper::RootNode;

use super::mutation::Mutation;
use super::query::Query;

pub type Schema = RootNode<'static, Query, Mutation>;
