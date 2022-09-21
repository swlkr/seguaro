use std::{collections::HashMap, io::Result};

use crate::node::Node;

pub struct Router {
    routes: HashMap<Method, Node<Handler>>,
}

pub type Handler = fn(String) -> Result<()>;

#[derive(PartialEq, Eq, Hash)]
pub enum Method {
    TRACE,
    HEAD,
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, method: &str, path: &str, handler: Handler) {}
}
