pub struct Cell<T> 
  where T: PartialEq {
  keys: Vec<T>,
}

impl<T: PartialEq> Cell<T> {
  fn new(size: usize) -> Cell<T> {
    Cell{
    keys: Vec::with_capacity(size)
    }
  }

  fn full(&self) -> bool {
    return self.keys.len() == self.keys.capacity()
  }

  fn contains(&self, key: &T) -> bool {
    self.keys.iter().any(|a| a == key)
  }
}