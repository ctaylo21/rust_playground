use std::collections::HashMap;

pub fn mean(numbers: &[i32]) -> f32 {
    let mut sum = 0;

    for x in numbers {
        sum += x;
    }

    sum as f32 / numbers.len() as f32
}

pub fn median(numbers: &mut [i32]) -> i32 {
    let mid: usize = numbers.len() / 2;

    numbers.sort();

    numbers[mid]
}

pub fn mode(numbers: &[i32]) -> i32 {
    let mut count = HashMap::new();

    let mut max_count = 0;
    let mut max_value = numbers[0];

    for x in numbers {
        let num_seen = count.entry(x).or_insert(0);
        *num_seen += 1;

        if *num_seen > max_count {
            max_count = *num_seen;
            max_value = *x;
        }
    }

    max_value
}

