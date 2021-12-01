fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (num_depth_increases, _) = input.split("\n").fold((-1, 0), |(mut sum, prev), d_str| {
        let d = d_str.parse::<i32>().unwrap();
        if d > prev { sum += 1; }
        (sum, d)
    });

    println!("Part 1: {}", num_depth_increases);
}
