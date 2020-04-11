use std::{error::Error, fmt};

#[derive(Debug)]
pub struct CellFull;

impl Error for CellFull {}

impl fmt::Display for CellFull {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tried to insert an item in a cell which was full")
    }
}

pub struct Cell<T>
where
    T: PartialEq + Clone,
{
    keys: Vec<T>,
}

impl<T: PartialEq + Clone> Cell<T> {
    pub fn new(size: usize) -> Cell<T> {
        Cell {
            keys: Vec::with_capacity(size),
        }
    }

    pub fn full(&self) -> bool {
        self.keys.len() == self.keys.capacity()
    }

    pub fn contains(&self, key: &T) -> bool {
        self.keys.iter().any(|a| a == key)
    }

    pub fn insert(&mut self, key: &T) -> Result<(), CellFull> {
        if self.full() {
            return Err(CellFull {});
        }
        self.keys.push(key.clone());
        Ok(())
    }
}
