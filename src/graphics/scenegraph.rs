use std::default::Default;
use std::rc::Rc;
use std::cell::RefCell;

trait Render {
    fn render(&mut self);
}

struct Node {
    children: Vec<Box<Node>>,
    parent: Option<Box<Node>>,
    object: Option<Rc<RefCell<Box<Render+'static>>>>,
}

impl Render for Option<Rc<RefCell<Box<Render+'static>>>> {
    fn render(&mut self) {
        self.unwrap().borrow().render();
    }
}

impl Node {
    fn new(object: Rc<RefCell<Box<Render+'static>>>, parent: Option<Box<Node>>) -> Node {
        Node {
            parent: parent,
            children: Vec::new(),
            object: Some(object),
        }
    }

    fn insert(&mut self, child: Box<Node>) {
        self.children.push(child);
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
        let testobj = box TestObj { rendered: false };
        let mut obj = Rc::new(RefCell::new(testobj));

        graph.parent.insert(box Node::new(obj.clone(), None));

        graph.render();

        assert_eq!(obj.borrow().rendered, true);
    }
}
