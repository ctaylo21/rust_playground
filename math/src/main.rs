mod math;

fn main() {
    let numbers_mean = vec![1, 4, 5, 4, 8, 6, 2, 6, 8, 2, 1];
    let mut numbers_median = vec![1, 4, 5, 4, 8, 6, 2, 6, 1, 3];
    let numbers_mode = vec![1, 4, 5, 4, 8, 6, 2, 6, 8, 2, 6];

    println!("Mean: {}", math::mean(&numbers_mean));
    println!("Median: {}", math::median(&mut numbers_median));
    println!("Mode: {}", math::mode(&numbers_mode));
}
