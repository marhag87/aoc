fn parse_instructions(input: &str) -> Vec<(usize, isize)> {
    let mut cycle: usize = 1;
    let mut x = 1;
    let mut history = vec![(cycle, x)];
    input.lines().for_each(|line| {
        let mut split = line.split_ascii_whitespace();
        match split.next() {
            Some("noop") => {
                // Do nothing, one cycle
                cycle += 1;
                history.push((cycle, x));
            }
            Some("addx") => {
                // Do add, two cycles
                let num = split
                    .next()
                    .expect("should have an input")
                    .parse::<isize>()
                    .expect("should be a number");
                cycle += 1;
                history.push((cycle, x));
                cycle += 1;
                x += num;
                history.push((cycle, x));
            }
            _ => panic!("invalid command"),
        }
    });
    history
}

pub(crate) fn day_10_1(input: &str) -> isize {
    parse_instructions(input)
        .into_iter()
        .enumerate()
        .filter_map(|(n, (cycle, x))| {
            if [20, 60, 100, 140, 180, 220].contains(&(n + 1)) {
                Some(cycle as isize * x)
            } else {
                None
            }
        })
        .sum()
}

pub(crate) fn day_10_2(input: &str) -> String {
    let mut output = String::from("");
    parse_instructions(input)
        .iter()
        .take(240)
        .enumerate()
        .for_each(|(n, (_, sprite_pos))| {
            let pixel_pos = n % 40;
            let diff = sprite_pos - pixel_pos as isize;
            if (-1..=1).contains(&diff) {
                output += "#";
            } else {
                output += ".";
            }
            if pixel_pos == 39 {
                output += "\n";
            }
        });
    output
}

#[test]
fn test_day_10_1() {
    let input = String::from_utf8_lossy(include_bytes!("../test_input/day_10.txt"));
    assert_eq!(day_10_1(&input), 13140);
}

#[test]
fn test_day_10_2() {
    let input = String::from_utf8_lossy(include_bytes!("../test_input/day_10.txt"));
    let expected = String::from(
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
",
    );
    assert_eq!(day_10_2(&input), expected);
}
