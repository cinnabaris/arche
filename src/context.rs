use super::cache::Cache;
use super::queue::Queue;
use super::repository::Repository;

pub struct Context {
    repository: Box<Repository>,
    cache: Box<Cache>,
    queue: Box<Queue>,
}

impl Context {
    pub fn new(r: Box<Repository>, c: Box<Cache>, q: Box<Queue>) -> Context {
        Context {
            repository: r,
            cache: c,
            queue: q,
        }
    }
}
