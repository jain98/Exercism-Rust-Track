// use maplit::hashmap;

pub mod graph {
    use crate::graph::graph_items::node::Node;
    use crate::graph::graph_items::edge::Edge;
    use std::collections::HashMap;
    use maplit::hashmap;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: hashmap!{},
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.iter().map(|n| n.clone()).collect();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.iter().map(|e| e.clone()).collect();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            attrs.iter().for_each(|(k, v)| {
                self.attrs.insert((*k).to_owned(), (*v).to_owned());
            });
            self
        }

        pub fn get_node(&self, n: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.n == n)
        }

        pub fn get_attr(&self, k: &str) -> Option<&str> {
            self.attrs.get(k).map(|v| &v[..])
        }

    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct Edge {
                e1: String,
                e2: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(e1: &str, e2: &str) -> Self {
                    Edge { e1: e1.to_owned(), e2: e2.to_owned(), attrs: HashMap::new() }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|(k, v)| {
                        self.attrs.insert((*k).to_owned(), (*v).to_owned());
                    });
                    self
                }

                pub fn get_attr(&self, k: &str) -> Option<&str> {
                    self.attrs.get(k).map(|v| &v[..])
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, Eq, PartialEq)]
            pub struct Node {
                pub(crate) n: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(n: &str) -> Self {
                    Node { n: n.to_owned(), attrs: HashMap::new() }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|(k, v)| {
                        self.attrs.insert((*k).to_owned(), (*v).to_owned());
                    });
                    self
                }

                pub fn get_attr(&self, k: &str) -> Option<&str> {
                    self.attrs.get(k).map(|v| &v[..])
                }
            }
        }


    }
}
