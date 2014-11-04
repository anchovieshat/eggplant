use std::default::Default;
use std::rc::Rc;
use std::cell::RefCell;

trait Render {
    fn render(&mut self);
}

struct Node {
    children: Vec<Box<Node>>,
    parent: Option<Box<Node>>,
    object: Option<Box<Render+'static>>,
}

impl Render for Option<Box<Render+'static>> {
    fn render(&mut self) {
        self.as_mut().unwrap().render()
    }
}

impl<T: Render> Render for Rc<RefCell<Box<T>>> {
    fn render(&mut self) {
        self.borrow_mut().render()
    }
}

impl Node {
    fn new(object: Box<Render+'static>, parent: Option<Box<Node>>) -> Node {
        Node {
            parent: parent,
            children: Vec::new(),
            object: Some(object),
        }
    }

    fn insert(&mut self, child: Box<Node>) {
        match self.object {
            Some(_) => self.children.push(child),
            None => *self = *child,
        }
    }

    fn render(&mut self) {
        self.object.render();
        for child in self.children.iter_mut() {
            child.render();
        }
    }
}

impl Default for Node {
    fn default() -> Node {
        Node {
            parent: None,
            children: Vec::new(),
            object: None
        }
    }
}

struct SceneGraph {
    parent: Box<Node>,
}

impl SceneGraph {
    fn new(parent: Box<Node>) -> SceneGraph {
        SceneGraph {
            parent: parent,
        }
    }

    fn render(&mut self) {
        self.parent.render();
    }
}

#[cfg(test)]
mod test {
    use super::{SceneGraph, Render, Node};
    use std::default::Default;
    use std::fmt::Show;
    use std::any::Any;
    use std::boxed::BoxAny;
    use std::rc::Rc;
    use std::cell::RefCell;

    struct TestObj {
        rendered: bool,
    }

    impl Render for TestObj {
        fn render(&mut self) { self.rendered = true; }
    }

    #[test]
    fn test_traversal() {
        let mut graph = SceneGraph::new(Default::default());
        let testobj = Rc::new(RefCell::new(box TestObj { rendered: false }));
        let testnode = box Node::new(box testobj.clone(), None);

        graph.parent.insert(testnode);

        graph.render();

        assert_eq!(testobj.borrow().rendered, true);
    }
}
