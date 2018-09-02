use std::rc::{ Rc, };
use std::cell::{ Cell, Ref, RefMut, RefCell, };


#[derive(Debug)]
pub struct RcRef<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> Clone for RcRef<T> {
    fn clone(&self) -> Self {
        RcRef {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<T> RcRef<T> {
    pub fn new(val: T) -> Self {
        RcRef { inner: Rc::new(RefCell::new(val)) }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}