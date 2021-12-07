fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", best_position(input.as_str()).1);
}

fn best_position(input: &str) -> (usize, usize) {
    let starts: Vec<usize> = input.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    let min = starts.clone().into_iter().min().unwrap();
    let max = starts.clone().into_iter().max().unwrap();
    (min..max).into_iter()
        .map(|pos| (pos, calculate_fuel(starts.clone(), pos)))
        .min_by(|(_, f1), (_, f2)| f1.cmp(f2)).unwrap()
}

fn calculate_fuel(starts: Vec<usize>, pos: usize) -> usize {
    starts.iter().map(|s| if s > &pos { s - pos } else { pos - s }).sum()
}

#[test]
fn test_part1() {
    let input = "16,1,2,0,4,2,7,1,2,14";
    assert_eq!(best_position(input).0, 2);
}
