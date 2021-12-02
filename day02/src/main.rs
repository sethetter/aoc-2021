enum Dir {
    Forward,
    Down,
    Up,
}
type Command = (u32, Dir);

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let cmds = parse_commands(input);

    let (final_x, final_z) = cmds.into_iter().fold((0, 0), |(x, z), (dist, dir)| match dir {
        Dir::Up => (x, z - dist),
        Dir::Down => (x, z + dist),
        Dir::Forward => (x + dist, z),
    });

    println!("Part 1: {}", final_x * final_z);
}

fn parse_commands(input: String) -> Vec<Command> {
    input.split("\n").fold(vec![], |mut cmds, line| {
        let mut parts = line.split(" ");

        let dir_str = parts.next().unwrap();
        let dir = dir_from_str(dir_str.to_owned());

        let dist_str = parts.next().unwrap();
        let dist: u32 = dist_str.parse().unwrap();

        cmds.push((dist, dir));
        cmds
    })
}

fn dir_from_str(str: String) -> Dir {
    match str.as_str() {
        "up" => Dir::Up,
        "down" => Dir::Down,
        "forward" => Dir::Forward,
        _ => panic!("Invalid dir_str!")
    }
}