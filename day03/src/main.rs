fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (gamma, epsilon) = gamma_and_epsilon(input.trim().split("\n").collect());
    println!("Part 1: {}", gamma * epsilon);

    let ogr = filter_for_value(0, input.clone());
    let csr = filter_for_value(1, input.clone());

    println!("Part 2: {}", ogr * csr);
}

fn filter_for_value(g_or_e: u8, input: String) -> u64 {
    let num_bits = input.clone().split("n").next().unwrap().len();

    let mut lines: Vec<&str> = input.split("\n").collect();
    for b in 0..num_bits {
        if lines.len() == 1 {
            break;
        }

        let (gamma, epsilon) = gamma_and_epsilon(lines.clone());
        let filter_bits = match g_or_e {
            0 => format!("{:012b}", gamma),
            1 => format!("{:012b}", epsilon),
            _ => unreachable!(),
        };

        lines = lines
            .clone()
            .into_iter()
            .filter(|l| l.chars().nth(b).unwrap() == filter_bits.chars().nth(b).unwrap())
            .collect();
    }

    if lines.len() > 1 {
        panic!("Filter didn't work!");
    }

    let str = lines.get(0).unwrap();
    u64::from_str_radix(str, 2).unwrap()
}

fn gamma_and_epsilon(lines: Vec<&str>) -> (u32, u32) {
    let mut arr: Vec<(u32, u32)> = vec![];
    let num_bits = lines.clone().into_iter().next().unwrap().trim().len();

    for _ in 0..num_bits {
        arr.push((0, 0));
    }
    for line in lines.clone() {
        let bits: Vec<String> = line.split("").map(|s| s.to_owned()).collect();
        for i in 0..num_bits {
            match bits.get(i + 1).unwrap().as_str() {
                "0" => arr[i].0 += 1,
                "1" => arr[i].1 += 1,
                _ => println!("UNEXPECTED BIT! in pos {}", i),
            }
        }
    }
    let mut gamma = String::from("");
    let mut epsilon = String::from("");
    for i in 0..num_bits {
        gamma.push(if arr[i].0 <= arr[i].1 { '1' } else { '0' });
        epsilon.push(if arr[i].0 <= arr[i].1 { '0' } else { '1' });
    }
    // combine the digits
    let gx = u32::from_str_radix(gamma.as_str(), 2).unwrap();
    let ex = u32::from_str_radix(epsilon.as_str(), 2).unwrap();
    (gx, ex)
}

#[test]
fn test() {
    let sample = std::fs::read_to_string("sample.txt").unwrap();
    assert_eq!(gamma_and_epsilon(sample.split("\n").collect()), (22, 9));
}
