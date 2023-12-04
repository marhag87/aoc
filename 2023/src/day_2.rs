fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_2.txt"));
    let first = day_2_1(&input);
    println!("{:?}", first);
    let second = day_2_2(&input);
    println!("{:?}", second);
}

fn day_2_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let split = line.split(": ").collect::<Vec<&str>>();
            let game = split[0];
            let game = game.split(' ').collect::<Vec<&str>>();
            let game = game[1].parse::<usize>().unwrap();

            let pulls = split[1].split("; ").collect::<Vec<&str>>();
            if pulls.iter().any(|pull| {
                let dice = pull.split(", ").collect::<Vec<&str>>();
                dice.iter().any(|die| {
                    let die_split = die.split(' ').collect::<Vec<&str>>();
                    let number = die_split[0].parse::<usize>().unwrap();
                    let color = die_split[1];
                    match color {
                        "red" => number > 12,
                        "green" => number > 13,
                        "blue" => number > 14,
                        _ => unreachable!(),
                    }
                })
            }) {
                0
            } else {
                game
            }
        })
        .sum()
}

fn day_2_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            let split = line.split(": ").collect::<Vec<&str>>();

            let pulls = split[1].split("; ").collect::<Vec<&str>>();
            pulls.iter().for_each(|pull| {
                let dice = pull.split(", ").collect::<Vec<&str>>();
                dice.iter().for_each(|die| {
                    let die_split = die.split(' ').collect::<Vec<&str>>();
                    let number = die_split[0].parse::<usize>().unwrap();
                    let color = die_split[1];
                    match color {
                        "red" => {
                            if number > min_red {
                                min_red = number;
                            }
                        }
                        "green" => {
                            if number > min_green {
                                min_green = number;
                            }
                        }
                        "blue" => {
                            if number > min_blue {
                                min_blue = number;
                            }
                        }
                        _ => unreachable!(),
                    }
                })
            });
            min_red * min_green * min_blue
        })
        .sum()
}

#[test]
fn test_day_2_1() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    assert_eq!(day_2_1(input), 8);
}

#[test]
fn test_day_2_2() {
    let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    assert_eq!(day_2_2(input), 2286);
}
