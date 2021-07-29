//! This data structure is essentially a buffed out linked list.
//!
//!
//!
use serde::{Serialize, Deserialize};
use std::{ops, fmt::{self, Write}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chain<T> {
    data: T,
    next: Option<Box<Chain<T>>>
}

impl<T> Chain<T> {

    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }

    pub fn push(&mut self, data: T) {
        let link = Self::new(data);
        
    }

    /// Get a reference to the chain's data.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Set the chain's data.
    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }

    /// Get a mutable reference to the chain's data.
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// Get a reference to the chain's next.
    pub fn next(&self) -> Option<&Box<Chain<T>>> {
        self.next.as_ref()
    }

    /// Get a mutable reference to the chain's next.
    pub fn next_mut(&mut self) -> &mut Option<Box<Chain<T>>> {
        &mut self.next
    }

    /// Set the chain's next.
    pub fn set_next(&mut self, next: Option<Box<Chain<T>>>) {
        self.next = next;
    }
}

impl<T> fmt::Display for Chain<T> 
where
    T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("CHAIN {}", self.data))
     } 
}

impl<T> ops::Deref for Chain<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data 
    }
}
