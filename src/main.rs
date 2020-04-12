mod hashmap;
mod dni;
use hashmap::{HashMap, HashType, ProbeType};
use dni::DNI;
use std::io;
use rand::random;

fn main() {
    println!("Introduce the size of the HashMap: ");
    let size = read_between(0, usize::max_value());
    println!("Introduce the size of the HashMap cells: ");
    let cell_size = read_between(0, usize::max_value());
    println!("Introduce the number of attempts: ");
    let attempts = read_between(10, usize::max_value());
    println!("Introduce the load factor: ");
    let load_factor: f64 = read_between(0.0, 1.0);
    let mut hashmap = HashMap::<DNI>::new(size, cell_size, HashType::Modulo, ProbeType::Lineal);
    
    let n: usize = (load_factor * size as f64 * cell_size as f64) as usize;
    let mut dnis = Vec::new();
    for _i in 0..2*n {
        dnis.push(DNI::new());
    }

    for i in 0..n {
        loop {
            if let Err(_) = hashmap.insert(&dnis[i]) {
                dnis[i] = DNI::new(); 
            } else {
                break;
            }
        }
    }
    DNI::reset_comparisons();

    let mut search_min = usize::max_value();
    let mut search_mean = 0.0;
    let mut search_max = 0;

    for _i in 0..attempts {
        let index: usize = random::<usize>() % n;
        hashmap.contains(&dnis[index]);
        let comparisons = DNI::get_comparisons();
        if search_min > comparisons {
            search_min = comparisons;
        }
        if search_max < comparisons {
            search_max = comparisons;
        }
        search_mean += comparisons as f64;
        DNI::reset_comparisons();
    }
    search_mean /= attempts as f64;

    let mut insert_min = usize::max_value();
    let mut insert_mean = 0.0;
    let mut insert_max = 0;

    for _i in 0..attempts {
        let index: usize = 2 * n - 1 - (random::<usize>() % n);
        hashmap.contains(&dnis[index]);
        let comparisons = DNI::get_comparisons();
        if insert_min > comparisons {
            insert_min = comparisons;
        }
        if insert_max < comparisons {
            insert_max = comparisons;
        }
        insert_mean += comparisons as f64;
        DNI::reset_comparisons();
    }
    insert_mean /= attempts as f64;

    println!("Search");
    println!("Min:{}\tMax:{}\tMean:{}", search_min, search_max, search_mean);
    println!("Insert");
    println!("Min:{}\tMax:{}\tMean:{}", insert_min, insert_max, insert_mean);
}

fn read_between<T>(lower: T, upper: T) -> T 
where
    T: std::str::FromStr + PartialOrd + std::fmt::Display + std::fmt::Debug,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    loop {
        let mut input = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            println!("There has been an error: {}", error);
            continue;
        };
        let input = input.trim().parse::<T>();
        match input {
            Ok(input) => {
                if input < lower || input > upper {
                    println!("Please introduce something between {} and {}", lower, upper);
                } else {
                    break input;
                }
            }
            Err(err) => {
                println!("Please introduce the correct type {:?}", err);
                continue;
            }
        }
    }
}
