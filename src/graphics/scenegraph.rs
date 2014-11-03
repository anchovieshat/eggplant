trait Render {
    fn render(&mut self);
}

trait Node : Render {
    fn insert(&mut self, obj: Box<Node + 'static>);
    fn remove<T: Node>(&mut self, obj: &T);
}

struct SceneGraph {
    children: Vec<&'static Node+'static>,
}

impl SceneGraph {
    fn new() -> SceneGraph {
        SceneGraph {
            children: Vec::new(),
        }
    }
}

impl Node for SceneGraph {
    fn insert(&mut self, obj: Box<Node + 'static>) {
        self.children.push(obj);
    }

    fn remove<T: Node>(&mut self, obj: &T) {
    }

}

impl Render for SceneGraph {
    fn render(&mut self) {
        for child in self.children.iter() {
            let ibox = *child;
            child.render();
        }
    }
}


#[cfg(test)]
mod test {
    use super::{SceneGraph};
    use std::fmt::Show;

    #[deriving(Clone)]
    struct TestNode {
        rendered: bool,
    }

    impl Render for TestNode {
        fn render(&mut self) { self.rendered = true; }
    }

    #[test]
    fn test_traversal() {
        let mut graph = SceneGraph::new();
        let mut test_node = box TestNode { rendered: false };

        graph.insert(test_node.clone());

        graph.render();

        assert_eq!(test_node.rendered, true);
    }
}
