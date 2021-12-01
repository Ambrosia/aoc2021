use std::fs;

fn main() {
    let input: String =
        fs::read_to_string("input.txt").expect("What the hell man where's the input eat my ass");
    let depths = input
        .lines()
        .map(|str| str.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // count_by would be pretty cool but whatever
    let increases = depths.windows(2).fold(0, |increases, pair| {
        if pair[0] < pair[1] {
            increases + 1
        } else {
            increases
        }
    });
    println!("increases: {}", increases);

    let first_window = depths.windows(3).map(|values| values.iter().sum::<i32>());
    let mut depths2 = depths.clone();
    depths2.remove(0);
    let second_window = depths2.windows(3).map(|values| values.iter().sum::<i32>());
    let increases2 = first_window.zip(second_window).fold(0, |increases, pair| {
        if pair.0 < pair.1 {
            increases + 1
        } else {
            increases
        }
    });
    println!(
        "increases with 3 measurement sliding window sum: {}",
        increases2
    );
}
