fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (gamma, epsilon) = power_consumption(input.trim().to_owned());
    println!("Part 1: {}", gamma * epsilon);
}

fn power_consumption(input: String) -> (u32, u32) {
    let num_bits = input.clone().split("\n").next().unwrap().trim().len();
    let mut arr: Vec<(u32, u32)> = vec![];
    for _ in 0..num_bits {
        arr.push((0, 0));
    }
    for line in input.split("\n") {
        let bits: Vec<String> = line.split("").map(|s| s.to_owned()).collect();
        for i in 0..num_bits {
            match bits.get(i+1).unwrap().as_str() {
                "0" => arr[i].0 += 1,
                "1" => arr[i].1 += 1,
                _ => {println!("UNEXPECTED BIT! in pos {}", i);},
            }
        }
    }
    let mut gamma = String::from("");
    let mut epsilon = String::from("");
    for i in 0..num_bits {
        if arr[i].0 < arr[i].1 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    // combine the digits
    let gx = u32::from_str_radix(gamma.as_str(), 2).unwrap();
    let ex = u32::from_str_radix(epsilon.as_str(), 2).unwrap();
    (gx, ex)
}

#[test]
fn test() {
    let sample = std::fs::read_to_string("sample.txt").unwrap();
    assert_eq!(power_consumption(sample.to_owned()), (22, 9));
}
