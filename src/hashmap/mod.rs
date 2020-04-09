mod cell;
use cell::Cell;

pub struct HashMap<T> 
  where T: PartialEq {
  cells: Vec<Cell<T>>,  
}

impl<T: PartialEq> HashMap<T> {
  fn new(size: usize) -> HashMap<T> {
    HashMap {
      cells: Vec::with_capacity(size),
    }
  }
}