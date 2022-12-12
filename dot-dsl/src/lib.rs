pub mod graph {
    use std::collections::HashMap;
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    
    #[derive(Debug, Default, PartialEq, Eq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }
    impl Graph {
        pub fn new() -> Self {
            Self { ..Default::default() }   
        }
        
        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.clone();
            self
        }
        pub fn node(&self, name: &str) -> Option<Node> {
            for n in &self.nodes {
                if n.name == name {
                    return Some(n.clone());
                }
            }
            None
        }
        
        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.clone();
            self
        }
        
        pub fn with_attrs(mut self, attrs:&[(&str, &str)]) -> Self {
            for a in attrs {
                let (attr, value) = a;
                self.attrs.insert(attr.to_string(), value.to_string());
            }
            self
        }
    
        pub fn attr(&self, key: &str) -> Option<&str> {
            match self.attrs.get(key) {
                Some(v) => Some(v.as_str()),
                None => None,
            }
        }
    }
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Edge {
                src: String,
                dst: String,
                attrs: HashMap<String, String>,
            }
            
            impl Edge {
                pub fn new(src: &str, dst: &str) -> Self {
                    Self {
                        src: src.to_owned(), 
                        dst: dst.to_owned(),
                        attrs: HashMap::new()
                    }
                }
                
                pub fn with_attrs(mut self, attrs:&[(&str, &str)]) -> Self {
                    for a in attrs {
                        let (attr, value) = a;
                        self.attrs.insert(attr.to_string(), value.to_string());
                    }
                    self
                }
                
                pub fn attr(&self, key: &str) -> Option<&str> {
                    match self.attrs.get(key) {
                        Some(v) => Some(v.as_str()),
                        None => None,
                    }
                }
            }
        }
        
        pub mod node {
            use std::collections::HashMap;
            
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }
            
            impl Node {
                pub fn new(name: &str) -> Self {
                    Self { 
                        name: name.to_owned(), 
                        attrs: HashMap::new()
                    }
                }
                
                pub fn with_attrs(mut self, attrs:&[(&str, &str)]) -> Self {
                    for a in attrs {
                        let (attr, value) = a;
                        self.attrs.insert(attr.to_string(), value.to_string());
                    }
                    self
                }
                
                pub fn attr(&self, key: &str) -> Option<&str> {
                     match self.attrs.get(key) {
                        Some(v) => Some(v.as_str()),
                        None => None,
                    }
                }
            }            
        }
    }
}
