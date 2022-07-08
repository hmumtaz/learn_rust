use std::collections::HashMap;

fn main() {
    let mut integers: [i32; 8] = [3, 5, 7, 12, 9, 6, 3, 4];
    let statistics = vec![
        calculate_mean(&integers),
        calculate_median(&mut integers),
        calculate_mode(&integers),
    ];
    println!("{:?}", statistics)
}

#[derive(Debug)]
enum Numeric {
    Int(i32),
    Float(f64),
}

fn calculate_mean(integers: &[i32]) -> Numeric {
    let mut mean: f64 = 0.0;
    for i in integers {
        mean = mean + *i as f64;
    }
    let size = integers.len() as f64;
    return Numeric::Float(mean / size);
}

fn calculate_median(integers: &mut [i32]) -> Numeric {
    integers.sort();
    let size = integers.len();
    let middle_index: usize = size / 2;
    return Numeric::Int(integers[middle_index]);
}

fn calculate_mode(integers: &[i32]) -> Numeric {
    let mut map = HashMap::new();
    for i in integers {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut ret = -1;
    let mut ret_val = -1;
    for (k, v) in map.iter() {
        if *v > ret_val {
            ret_val = *v;
            ret = *k;
        }
    }
    return Numeric::Int(ret);
}
