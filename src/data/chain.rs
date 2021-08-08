//! This data structure is essentially a buffed out linked list.
//! greatly inspired by learning rust with entirely too many linked list (rust-unofficial on gh)
//!
//!
// use serde::{Serialize, Deserialize};
use std::{fmt::{self, Write}, ops, rc::Rc};

#[derive(Debug, Clone)]
pub struct Chain<T> {
    head: Option<Rc<Link<T>>>
}

impl<T> std::ops::Deref for Chain<T> {
    type Target = Option<Rc<Link<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.head
    }
}

#[derive(Debug, Clone)]
pub struct Link<T> {
    data: T,
    next: Option<Rc<Link<T>>>,
}

impl<T> std::ops::Deref for Link<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Link<T> {

    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }

    pub fn init(data: T, next: Option<Rc<Link<T>>>) -> Self {
        Self { data, next }
    }

    /// Get a mutable reference to the link's next.
    pub fn next_mut(&mut self) -> &mut Option<Rc<Link<T>>> {
        &mut self.next
    }
    /// Get a reference to the link's next.
    pub fn next(&self) -> Option<&Rc<Link<T>>> {
        self.next.as_ref()
    }

    /// Get a mutable reference to the link's data.
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// Get a reference to the link's data.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Set the link's data.
    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }

    /// Set the link's next.
    pub fn set_next(&mut self, next: Option<Rc<Link<T>>>) {
        self.next = next;
    }
}


impl<T> Chain<T> {

    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn init(data: T) -> Self {
        Self { head: Some(Rc::new(Link::new(data))) }
    }

    pub fn prepend(&mut self, data: T) {
        self.head = Some(Rc::new(Link::init(data, self.head.clone())));
    }
    pub fn tail(&self) -> Chain<T> {
        Chain { head: self.head.as_ref().and_then(|node| node.next.clone())}
    }
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
    pub fn iter(&self) -> ChainIter<'_, T> {
        ChainIter { next: self.head.as_deref() }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        self.head.as_ref()
            .and_then(|n| {
                len += 1;
                n.next.clone()
            });
        return len;
    }

    /// Get a mutable reference to the chain's head.
    pub fn head_mut(&mut self) -> &mut Option<Rc<Link<T>>> {
        &mut self.head
    }

    /// Set the chain's head.
    pub fn set_head(&mut self, head: Option<Rc<Link<T>>>) {
        self.head = head;
    }
}
pub struct ChainIter<'a, T> {
    next: Option<&'a Link<T>>
}
impl<'a, T> Iterator for ChainIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}
impl<T> Drop for Chain<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(mut node) = head {
            match Rc::try_unwrap(node) {
                Ok(mut n) => { head = n.next.take(); }
                Err(e) => break,
            }
        }
    }
}
/* impl<'a, T> IntoIterator for Chain<T> 
where
    T: Sized + Clone
{
    type Item = &'a T;
    type IntoIter = ChainIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
} */

impl<T> fmt::Display for Chain<T> 
where
    T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("CHAIN {}", ""))
     } 
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::RecolResult;

    #[test]
    fn init_ok() -> RecolResult<()> {
        let _chain = Chain::<usize>::new();
        Ok(())
    }

    #[test]
    fn push_ok() -> RecolResult<()> {
        let mut chain = Chain::<usize>::new();
        /* chain.push(4);
        chain.push(8);
        chain.push(9); */
        debug_assert_eq!(chain.len(), 0);
        Ok(())
    }

    #[test]
    fn iter_ok() -> RecolResult<()> {
        Ok(())

    }
}
