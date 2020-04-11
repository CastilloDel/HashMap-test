mod cell;
use cell::Cell;
use rand::random;
use std::{error::Error, fmt};

pub enum HashType {
    Modulo,
    Sum,
    Pseudorandom,
}

pub enum ProbeType {
    Lineal,
    Quadratic,
    DoubleHash,
}

#[derive(Debug)]
pub struct HashMapFull;

impl Error for HashMapFull {}

impl fmt::Display for HashMapFull {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The item couldn't be introduced in the Hashmap")
    }
}

pub struct HashMap<T>
where
    T: PartialEq + Into<usize> + Clone,
{
    cells: Vec<Cell<T>>,
    hash: Box<dyn Fn(&T) -> usize>,
    probe: Box<dyn Fn(&T, usize) -> usize>,
}

impl<T> HashMap<T>
where
    T: PartialEq + Into<usize> + Clone,
{
     pub fn new(
        size: usize,
        cell_size: usize,
        hash_type: HashType,
        probe_type: ProbeType,
    ) -> HashMap<T> {
        let mut vec = Vec::with_capacity(size);
        for _i in 0..size {
            vec.push(Cell::new(cell_size));
        }
        let hash_fn: Box<dyn Fn(&T) -> usize> = match hash_type {
            HashType::Modulo => Box::new(|key| key.clone().into()),
            HashType::Sum => {
                Box::new(|key| {
                    let mut total = key.clone().into();
                    let mut sum = 0;
                    while total > 0 {
                        sum += total % 10;
                        total /= 10;
                    }
                    sum
                })
            }
            HashType::Pseudorandom => {
                Box::new(|_| {
                    random()
                })
            }
        };
        let probe_fn: Box<dyn Fn(&T, usize) -> usize> = match probe_type {
            ProbeType::Lineal => Box::new(|_, i| i),
            ProbeType::Quadratic => Box::new(|_, i| i * i),
            ProbeType::DoubleHash => Box::new(|_, i| i * random::<usize>()),
        };
        HashMap { 
           cells: vec,
            hash: hash_fn,
            probe: probe_fn
        }
    }

    pub fn contains(&self, key: &T) -> bool {
        for i in 0..self.cells.len() {
            let index = ((self.hash)(key) + (self.probe)(key, i)) % self.cells.len();
            if self.cells[index].contains(&key) {
                return true;
            } else if !self.cells[index].full() {
                return false;
            }
        }
        false
    }

    pub fn insert(&mut self, key: &T) -> Result<(), HashMapFull> {
        for i in 0..self.cells.len() {
            let index = ((self.hash)(key) + (self.probe)(key, i)) % self.cells.len();
            if let Ok(()) = self.cells[index].insert(key) {
                return Ok(());
            }
        }
        Err(HashMapFull)
    }
}