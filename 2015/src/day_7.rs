use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum Target {
    Register(String),
    Direct(usize),
}

impl Target {
    fn new(s: &str, n: usize) -> Self {
        match s.split(' ').nth(n).unwrap().parse::<usize>() {
            Ok(num) => Target::Direct(num),
            Err(_) => Target::Register(s.split(' ').nth(n).unwrap().to_string()),
        }
    }
}

#[derive(Debug)]
enum Command {
    Rshift(Target, Target, Target),
    Lshift(Target, Target, Target),
    Or(Target, Target, Target),
    Direct(Target, Target),
    Not(Target, Target),
    And(Target, Target, Target),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s
            .split(' ')
            .take(2)
            .collect_tuple()
            .expect("should have tuples")
        {
            (_, "RSHIFT") => {
                Command::Rshift(Target::new(s, 0), Target::new(s, 2), Target::new(s, 4))
            }
            (_, "LSHIFT") => {
                Command::Lshift(Target::new(s, 0), Target::new(s, 2), Target::new(s, 4))
            }
            (_, "OR") => Command::Or(Target::new(s, 0), Target::new(s, 2), Target::new(s, 4)),
            (_, "->") => Command::Direct(Target::new(s, 0), Target::new(s, 2)),
            ("NOT", _) => Command::Not(Target::new(s, 1), Target::new(s, 3)),
            (_, "AND") => Command::And(Target::new(s, 0), Target::new(s, 2), Target::new(s, 4)),
            _ => {
                panic!("invalid command")
            }
        }
    }
}

pub(crate) fn day_7_1(input: &str) -> usize {
    let commands = input
        .lines()
        .map(|line| line.into())
        .collect::<Vec<Command>>();
    let mut registers: HashMap<&String, Option<usize>> = HashMap::new();
    commands.iter().for_each(|command| match command {
        Command::Rshift(Target::Register(a), _, Target::Register(b)) => {
            registers.insert(a, None);
            registers.insert(b, None);
        }
        Command::Lshift(Target::Register(a), _, Target::Register(b)) => {
            registers.insert(a, None);
            registers.insert(b, None);
        }
        Command::Or(Target::Register(a), Target::Register(b), Target::Register(c)) => {
            registers.insert(a, None);
            registers.insert(b, None);
            registers.insert(c, None);
        }
        Command::Direct(Target::Register(a), Target::Register(b)) => {
            registers.insert(a, None);
            registers.insert(b, None);
        }
        Command::Direct(Target::Direct(_), Target::Register(a)) => {
            registers.insert(a, None);
        }
        Command::Not(Target::Register(a), Target::Register(b)) => {
            registers.insert(a, None);
            registers.insert(b, None);
        }
        Command::And(Target::Register(a), Target::Register(b), Target::Register(c)) => {
            registers.insert(a, None);
            registers.insert(b, None);
            registers.insert(c, None);
        }
        Command::And(Target::Direct(_), Target::Register(a), Target::Register(b)) => {
            registers.insert(a, None);
            registers.insert(b, None);
        }
        _ => {
            panic!("invalid command");
        }
    });
    while registers.iter().any(|(_, v)| v.is_none()) {
        commands.iter().for_each(|command| match command {
            Command::Rshift(Target::Register(a), Target::Direct(b), Target::Register(c)) => {
                if let Some(Some(a)) = registers.get(a) {
                    registers.insert(c, Some(a >> b));
                };
            }
            Command::Lshift(Target::Register(a), Target::Direct(b), Target::Register(c)) => {
                if let Some(Some(a)) = registers.get(a) {
                    registers.insert(c, Some(a << b));
                };
            }
            Command::Or(Target::Register(a), Target::Register(b), Target::Register(c)) => {
                if let Some(Some(a)) = registers.get(a) {
                    if let Some(Some(b)) = registers.get(b) {
                        registers.insert(c, Some(a | b));
                    }
                };
            }
            Command::Direct(Target::Register(a), Target::Register(b)) => {
                if let Some(a) = registers.get(a) {
                    registers.insert(b, a.clone());
                };
            }
            Command::Direct(Target::Direct(a), Target::Register(b)) => {
                registers.insert(b, Some(a.clone()));
            }
            Command::Not(Target::Register(a), Target::Register(b)) => {
                if let Some(Some(a)) = registers.get(a) {
                    registers.insert(b, Some(!a.clone()));
                };
            }
            Command::And(Target::Register(a), Target::Register(b), Target::Register(c)) => {
                if let Some(Some(a)) = registers.get(a) {
                    if let Some(Some(b)) = registers.get(b) {
                        registers.insert(c, Some(a & b));
                    }
                };
            }
            Command::And(Target::Direct(a), Target::Register(b), Target::Register(c)) => {
                if let Some(Some(b)) = registers.get(b) {
                    registers.insert(c, Some(a & b));
                }
            }
            _ => {
                eprintln!("command = {:?}", command);
                panic!("help")
            }
        });
    }
    registers.get(&"a".to_string()).unwrap().unwrap()
}
