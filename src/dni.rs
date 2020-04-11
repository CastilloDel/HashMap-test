use std::sync::atomic::{AtomicUsize, Ordering};
use rand::random;

static COMPARISONS: std::sync::atomic::AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct DNI {
  val: usize,
}

impl PartialEq for DNI {
  fn eq(&self, other: &DNI) -> bool {
    COMPARISONS.swap(COMPARISONS.load(Ordering::SeqCst) + 1, Ordering::SeqCst);
    self.val == other.val
  }
}

impl From<DNI> for usize {
  fn from(dni: DNI) -> Self {
    dni.val
  }
}

impl DNI {
  pub fn new() -> DNI {
    DNI {
      val: random(),
    }
  }

  pub fn get_comparisons() -> usize {
    COMPARISONS.load(Ordering::SeqCst)
  }

  pub fn reset_comparisons() {
    COMPARISONS.swap(0, Ordering::SeqCst);
  }
}