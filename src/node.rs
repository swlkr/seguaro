#[derive(Debug)]
pub struct Node<F> {
    pub nodes: Vec<Node<F>>,
    pub key: String,
    pub handler: Option<F>,
    pub is_param: bool,
}

impl<F> Node<F> {
    pub fn new(key: &str) -> Self {
        Node {
            nodes: Vec::new(),
            key: String::from(key),
            handler: None,
            is_param: key.starts_with('{') && key.ends_with('}'),
        }
    }

    pub fn insert(&mut self, path: &str, f: F) {
        match path.split_once('/') {
            Some((root, "")) => {
                self.key = String::from(root);
                self.handler = Some(f);
            }
            Some(("", path)) => self.insert(path, f),
            Some((root, path)) => {
                let node = self.nodes.iter_mut().find(|m| root == &m.key || m.is_param);
                match node {
                    Some(n) => n.insert(path, f),
                    None => {
                        let mut node = Node::new(root);
                        node.insert(path, f);
                        self.nodes.push(node);
                    }
                }
            }
            None => {
                let mut node = Node::new(path);
                node.handler = Some(f);
                self.nodes.push(node);
            }
        }
    }

    pub fn get(&self, path: &str) -> Option<&F> {
        match path.split_once('/') {
            Some((root, "")) => {
                if root == &self.key || self.is_param {
                    self.handler.as_ref()
                } else {
                    None
                }
            }
            Some(("", path)) => self.get(path),
            Some((root, path)) => {
                let node = self.nodes.iter().find(|m| root == &m.key || m.is_param);
                if let Some(node) = node {
                    node.get(path)
                } else {
                    None
                }
            }
            None => {
                let node = self.nodes.iter().find(|m| path == &m.key || m.is_param);
                if let Some(node) = node {
                    node.handler.as_ref()
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::Handler;

    #[test]
    fn test_get_route() {
        let mut root: Node<Handler> = Node::new("");
        root.insert("/", |_| Ok(()));
        root.insert("/a/b", |_| Ok(()));
        root.insert("/a/b/c", |_| Ok(()));
        root.insert("/a", |_| Ok(()));
        root.insert("/a/{id}", |_| Ok(()));
        root.insert("/posts/{id}/edit", |_| Ok(()));

        assert!(root.get("/").is_some());
        assert!(root.get("/a/b").is_some());
        assert!(root.get("/a/b/c").is_some());
        assert!(root.get("/a/123").is_some());
        assert!(root.get("/posts/123/edit").is_some());
    }
}
