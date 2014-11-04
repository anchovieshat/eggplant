use std::default::Default;
use std::rc::Rc;
use std::cell::RefCell;

trait Render {
    fn render(&mut self);
}

struct Dummy;

impl Dummy {
    fn new() -> Dummy { Dummy }
}

impl Render for Dummy {
    fn render(&mut self) { }
}

struct Node {
    id: u32,
    children: Vec<Rc<RefCell<Node>>>,
    object: Box<Render+'static>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id
    }
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

static mut NODE_ID: u32 = 0;


fn incr_node_id() {
    unsafe {
        NODE_ID = NODE_ID+1;
    }
}

fn get_node_id() -> u32 {
    unsafe {
        NODE_ID
    }
}

impl Node {
    fn new(object: Box<Render+'static>) -> Node {
        incr_node_id();
        Node {
            id: get_node_id(),
            children: Vec::new(),
            object: object,
        }
    }

    fn insert(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(child);
    }

    fn render(&mut self) {
        self.object.render();
        for child in self.children.iter_mut() {
            child.borrow_mut().render();
        }
    }

    fn find(&mut self, obj: &Node) -> Option<uint> {
        for (idx, child) in self.children.iter().enumerate() {
            if *child.borrow() == *obj {
               return Some(idx)
            }
        }
        None
    }
}

impl Default for Node {
    fn default() -> Node {
        incr_node_id();
        Node {
            id: get_node_id(),
            children: Vec::new(),
            object: box Dummy::new(),
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

    impl Default for TestObj {
        fn default() -> TestObj {
            TestObj {
                rendered: false,
            }
        }
    }

    #[test]
    fn test_traversal() {
        let mut graph = SceneGraph::new(Default::default());
        let testobj_l: TestObj = Default::default();
        let testobj = Rc::new(RefCell::new(box testobj_l));
        let testnode = Rc::new(RefCell::new(Node::new(box testobj.clone())));

        graph.parent.insert(testnode);

        graph.render();

        assert_eq!(testobj.borrow().rendered, true);
    }

    #[test]
    fn test_find_node() {
        let obj1: TestObj= Default::default();
        let obj2: TestObj= Default::default();
        let obj3: TestObj= Default::default();
        let mut node = Node::new(box obj1);
        let node2 = Rc::new(RefCell::new(Node::new(box obj2)));
        let node3 = Rc::new(RefCell::new(Node::new(box obj3)));

        node.insert(node2.clone());
        node.insert(node3.clone());

        assert_eq!(node.find(&*node3.borrow()), Some(1));
    }
}
