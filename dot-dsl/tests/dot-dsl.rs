use dot_dsl::graph::graph_items::edge::Edge;
use dot_dsl::graph::graph_items::node::Node;
use dot_dsl::graph::Graph;
use maplit::hashmap;

use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}


impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type `!Unpin`
        }
    }
    fn init<'a>(self: Pin<&'a mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a<'a>(self: Pin<&'a Self>) -> &'a str {
        &self.get_ref().a
    }

    fn b<'a>(self: Pin<&'a Self>) -> &'a String {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &*(self.b) }
    }
}


#[test]
fn dummy() {
    // test1 is safe to move before we initialize it
    let mut test1 = Test::new("test1");
    // Notice how we shadow `test1` to prevent it from being accessed again
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());
    unsafe {
        println!("x: {}, y: {}", test1.a, &*test1.b);

        let mut some_test = Test::new("Hello");
        some_test.b = &some_test.a;
        println!("s: {}, t: {}", some_test.a, &*some_test.b);
        let test2 = std::mem::replace(test1.get_mut(), some_test);
        println!("a: {}, b: {}", test2.a, &*test2.b);
    }
}

#[test]
fn test_empty_graph() {
    let graph = Graph::new();

    assert!(graph.nodes.is_empty());

    assert!(graph.edges.is_empty());

    assert!(graph.attrs.is_empty());
}

#[test]
#[ignore]
fn test_graph_with_one_node() {
    let nodes = vec![Node::new("a")];

    let graph = Graph::new().with_nodes(&nodes);

    assert!(graph.edges.is_empty());

    assert!(graph.attrs.is_empty());

    assert_eq!(graph.nodes, vec![Node::new("a")]);
}

#[test]
#[ignore]
fn test_graph_with_one_node_with_keywords() {
    let nodes = vec![Node::new("a").with_attrs(&[("color", "green")])];

    let graph = Graph::new().with_nodes(&nodes);

    assert!(graph.edges.is_empty());

    assert!(graph.attrs.is_empty());

    assert_eq!(
        graph.nodes,
        vec![Node::new("a").with_attrs(&[("color", "green")])]
    );
}

#[test]
#[ignore]
fn test_graph_with_one_edge() {
    let edges = vec![Edge::new("a", "b")];

    let graph = Graph::new().with_edges(&edges);

    assert!(graph.nodes.is_empty());

    assert!(graph.attrs.is_empty());

    assert_eq!(graph.edges, vec![Edge::new("a", "b")]);
}

#[test]
#[ignore]
fn test_graph_with_one_attribute() {
    let graph = Graph::new().with_attrs(&[("foo", "1")]);

    let expected_attrs = hashmap! {
        "foo".to_string() => "1".to_string(),
    };

    assert!(graph.nodes.is_empty());

    assert!(graph.edges.is_empty());

    assert_eq!(graph.attrs, expected_attrs);
}

#[test]
#[ignore]
fn test_graph_with_attributes() {
    let nodes = vec![
        Node::new("a").with_attrs(&[("color", "green")]),
        Node::new("c"),
        Node::new("b").with_attrs(&[("label", "Beta!")]),
    ];

    let edges = vec![
        Edge::new("b", "c"),
        Edge::new("a", "b").with_attrs(&[("color", "blue")]),
    ];

    let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];

    let expected_attrs = hashmap! {
        "foo".to_string() => "1".to_string(),
        "title".to_string() => "Testing Attrs".to_string(),
        "bar".to_string() => "true".to_string(),
    };

    let graph = Graph::new()
        .with_nodes(&nodes)
        .with_edges(&edges)
        .with_attrs(&attrs);

    assert_eq!(
        graph.nodes,
        vec![
            Node::new("a").with_attrs(&[("color", "green")]),
            Node::new("c"),
            Node::new("b").with_attrs(&[("label", "Beta!")]),
        ]
    );

    assert_eq!(
        graph.edges,
        vec![
            Edge::new("b", "c"),
            Edge::new("a", "b").with_attrs(&[("color", "blue")]),
        ]
    );

    assert_eq!(graph.attrs, expected_attrs);
}

#[test]
#[ignore]
fn test_graph_stores_attributes() {
    let attributes = [("foo", "bar"), ("bat", "baz"), ("bim", "bef")];
    let graph = Graph::new().with_nodes(
        &["a", "b", "c"]
            .iter()
            .zip(attributes.iter())
            .map(|(name, &attr)| Node::new(&name).with_attrs(&[attr]))
            .collect::<Vec<_>>(),
    );

    assert_eq!(
        graph
            .get_node("c")
            .expect("node must be stored")
            .get_attr("bim"),
        Some("bef")
    );
}
