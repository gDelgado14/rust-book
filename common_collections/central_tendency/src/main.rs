extern crate rand;

use rand::Rng;
use std::collections::HashMap;

// Given a list of integers,
// use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here)
// of the list.

fn avg(v: &[i32]) -> f32 {
    let sum = v
        .iter()
        .fold(0.0, |a, e| a + *e as f32);

    sum / v.len() as f32
}

fn median(v: &[i32]) -> f32 {
    let last_element_index = v.len() - 1;

    if v.len() % 2 == 0 {
        let left_val = v[last_element_index / 2];
        let right_val = v[(last_element_index / 2) + 1];

        avg(&vec![left_val, right_val])
    } else {
        let middle = (last_element_index as f32 / 2.0).ceil() as usize;

        v[middle] as f32
    }
}


fn mode(v: &[i32]) -> Option<i32> {
    let mut map = HashMap::new();

    for el in v {
        let count = map.entry(el).or_insert(0);
        *count += 1;
    }


    map.into_iter()
       .max_by(|(ak, av), (bk, bv)| bv.cmp(av))
       .map(|(k, v)| *k)
}


fn main() {
    let mut vec = Vec::new();

    for _ in 0..100 {
        let val = rand::thread_rng().gen_range(1, 101);
        vec.push(val);
    }

    vec.sort();

    let mean = avg(&vec);
    let median = median(&vec);
    let mode = mode(&vec);

    println!("Mean: {}, Median: {}", mean, median);

    match mode {
        Some(v) => println!("Mode: {}", v),
        None => println!("No mode")
    }
}
