use std::cell::RefCell;
use std::rc::Rc;

trait Render {
    fn render(&mut self);
}

impl<T: Render> Render for Rc<RefCell<T>> {
    fn render(&mut self) { self.borrow_mut().render(); }
}

trait Node : Render {
    fn insert(&mut self, obj: Box<Node + 'static>) { }
    fn find(&mut self, obj: &Node) -> Option<uint> { None }
    fn remove(&mut self, idx: uint) { }
}

impl<T: Node> Node for Rc<RefCell<T>> {
    fn insert(&mut self, obj: Box<Node + 'static>) { self.borrow_mut().insert(obj) }
    fn find(&mut self, obj: &Node) -> Option<uint> { self.borrow_mut().find(obj) }
    fn remove(&mut self, idx: uint) { self.borrow_mut().remove(idx) }
}

struct SceneGraph {
    children: Vec<Box<Node + 'static>>
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

    fn find(&mut self, obj: &Node) -> Option<uint> {
        None
    }

    fn remove(&mut self, idx: uint) {
    }

}

impl Render for SceneGraph {
    fn render(&mut self) {
        for child in self.children.iter_mut() {
            child.render();
        }
    }
}


#[cfg(test)]
mod test {
    use super::{SceneGraph, Render, Node};
    use std::fmt::Show;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct TestNode {
        rendered: bool,
    }

    impl Render for TestNode {
        fn render(&mut self) { self.rendered = true; }
    }

    impl Node for TestNode { }

    #[test]
    fn test_traversal() {
        let mut graph = SceneGraph::new();
        let mut test_node = Rc::new(RefCell::new(TestNode { rendered: false }));

        graph.insert(box test_node.clone());

        graph.render();

        assert_eq!(test_node.borrow_mut().rendered, true);
    }
}
