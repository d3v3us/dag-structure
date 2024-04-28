use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::{Display, Formatter},
    rc::Rc,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Pointer<T>(pub Option<Rc<RefCell<T>>>);

impl<T> Clone for Pointer<T> {
    fn clone(&self) -> Self {
        if let Some(val) = self.0.as_ref() {
            return Pointer(Some(val.clone()));
        }
        Pointer(None)
    }
}

impl<T> Pointer<T> {
    pub fn new(d: T) -> Pointer<T> {
        Pointer(Some(Rc::new(RefCell::new(d))))
    }
    pub fn as_ref(&self) -> Ref<T> {
        let res = self.0.as_ref().unwrap().as_ref().borrow();
        res
    }
    pub fn as_mut_ref(&self) -> RefMut<T> {
        let res = self.0.as_ref().unwrap().as_ref().borrow_mut();
        res
    }
    pub fn is_clone_of(&self, other: &Pointer<T>) -> bool {
        Rc::ptr_eq(&self.0.as_ref().unwrap(), &other.0.as_ref().unwrap())
    }
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }
}
#[derive(Debug, PartialEq, Eq)]

pub struct Node {
    pub val: i128,
    pub left: Pointer<Node>,
    pub right: Pointer<Node>,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn display_with_depth(node: &Node, f: &mut Formatter<'_>, depth: usize) -> std::fmt::Result {
            write!(f, "{:indent$}[{} \n", "", node.val, indent = depth * 4)?;
            if !node.left.is_none() {
                display_with_depth(&node.left.as_ref(),f, depth + 1)?;
            }
            if !node.right.is_none() {
                display_with_depth(&node.right.as_ref(),f, depth + 1)?;
            }
            write!(f, "{:indent$}]\n", "", indent = depth * 4)
        }

        display_with_depth(self,f, 0)
    }
}

impl Node {
    pub fn new(val: i128) -> Self {
        Node {
            val,
            left: Pointer(None),
            right: Pointer(None),
        }
    }
}
impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            val: self.val,
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}
