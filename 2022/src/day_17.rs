use std::{collections::HashMap, time::Instant};

#[derive(Debug, Clone, Copy)]
enum Shape {
    Minus,
    Plus,
    Hook,
    Pipe,
    Square,
}

#[derive(Debug, Clone)]
struct Rock {
    shape: Shape,
    x: usize,
    y: usize,
}

impl Rock {
    fn new(shape: Shape) -> Self {
        Self { shape, x: 2, y: 0 }
    }
    fn down(&mut self, chamber: &HashMap<(usize, usize), bool>) -> bool {
        if let true = match self.shape {
            Shape::Minus => {
                chamber.get(&(self.x, self.y - 1)).is_none()
                    && chamber.get(&(self.x + 1, self.y - 1)).is_none()
                    && chamber.get(&(self.x + 2, self.y - 1)).is_none()
                    && chamber.get(&(self.x + 3, self.y - 1)).is_none()
            }
            Shape::Plus => {
                chamber.get(&(self.x, self.y)).is_none()
                    && chamber.get(&(self.x + 1, self.y - 1)).is_none()
                    && chamber.get(&(self.x + 2, self.y)).is_none()
            }
            Shape::Hook => {
                chamber.get(&(self.x, self.y - 1)).is_none()
                    && chamber.get(&(self.x + 1, self.y - 1)).is_none()
                    && chamber.get(&(self.x + 2, self.y - 1)).is_none()
            }
            Shape::Pipe => chamber.get(&(self.x, self.y - 1)).is_none(),
            Shape::Square => {
                chamber.get(&(self.x, self.y - 1)).is_none()
                    && chamber.get(&(self.x + 1, self.y - 1)).is_none()
            }
        } {
            self.y -= 1;
            true
        } else {
            false
        }
    }
    fn left(&mut self, chamber: &HashMap<(usize, usize), bool>) {
        if self.x < 1 {
            return;
        }
        // println!("left");
        if let true = match self.shape {
            Shape::Minus => chamber.get(&(self.x - 1, self.y)).is_none(),
            Shape::Plus => {
                chamber.get(&(self.x, self.y)).is_none()
                    && chamber.get(&(self.x - 1, self.y + 1)).is_none()
                    && chamber.get(&(self.x, self.y + 2)).is_none()
            }
            Shape::Hook => {
                chamber.get(&(self.x - 1, self.y)).is_none()
                    && chamber.get(&(self.x + 1, self.y + 1)).is_none()
                    && chamber.get(&(self.x + 1, self.y + 2)).is_none()
            }
            Shape::Pipe => {
                chamber.get(&(self.x - 1, self.y)).is_none()
                    && chamber.get(&(self.x - 1, self.y + 1)).is_none()
                    && chamber.get(&(self.x - 1, self.y + 2)).is_none()
                    && chamber.get(&(self.x - 1, self.y + 3)).is_none()
            }
            Shape::Square => {
                chamber.get(&(self.x - 1, self.y)).is_none()
                    && chamber.get(&(self.x - 1, self.y + 1)).is_none()
            }
        } {
            if self.x > 0 {
                self.x -= 1;
            }
        }
    }
    fn right(&mut self, chamber: &HashMap<(usize, usize), bool>) {
        // println!("right");
        if let true = match self.shape {
            Shape::Minus => chamber.get(&(self.x + 4, self.y)).is_none() && self.x < 3,
            Shape::Plus => {
                chamber.get(&(self.x + 2, self.y)).is_none()
                    && chamber.get(&(self.x + 3, self.y + 1)).is_none()
                    && chamber.get(&(self.x + 2, self.y + 2)).is_none()
                    && self.x < 4
            }
            Shape::Hook => {
                chamber.get(&(self.x + 3, self.y)).is_none()
                    && chamber.get(&(self.x + 3, self.y + 1)).is_none()
                    && chamber.get(&(self.x + 3, self.y + 2)).is_none()
                    && self.x < 4
            }
            Shape::Pipe => {
                chamber.get(&(self.x + 1, self.y)).is_none()
                    && chamber.get(&(self.x + 1, self.y + 1)).is_none()
                    && chamber.get(&(self.x + 1, self.y + 2)).is_none()
                    && chamber.get(&(self.x + 1, self.y + 3)).is_none()
                    && self.x < 6
            }
            Shape::Square => {
                chamber.get(&(self.x + 2, self.y)).is_none()
                    && chamber.get(&(self.x + 2, self.y + 1)).is_none()
                    && self.x < 5
            }
        } {
            self.x += 1;
        }
    }
    fn settle(&self, chamber: &mut HashMap<(usize, usize), bool>) {
        // println!("settle: ({}, {})", self.x, self.y);
        match self.shape {
            Shape::Minus => {
                chamber.insert((self.x, self.y), true);
                chamber.insert((self.x + 1, self.y), true);
                chamber.insert((self.x + 2, self.y), true);
                chamber.insert((self.x + 3, self.y), true);
            }
            Shape::Plus => {
                chamber.insert((self.x + 1, self.y), true);
                chamber.insert((self.x, self.y + 1), true);
                chamber.insert((self.x + 1, self.y + 1), true);
                chamber.insert((self.x + 2, self.y + 1), true);
                chamber.insert((self.x + 1, self.y + 2), true);
            }
            Shape::Hook => {
                chamber.insert((self.x, self.y), true);
                chamber.insert((self.x + 1, self.y), true);
                chamber.insert((self.x + 2, self.y), true);
                chamber.insert((self.x + 2, self.y + 1), true);
                chamber.insert((self.x + 2, self.y + 2), true);
            }
            Shape::Pipe => {
                chamber.insert((self.x, self.y), true);
                chamber.insert((self.x, self.y + 1), true);
                chamber.insert((self.x, self.y + 2), true);
                chamber.insert((self.x, self.y + 3), true);
            }
            Shape::Square => {
                chamber.insert((self.x, self.y), true);
                chamber.insert((self.x + 1, self.y), true);
                chamber.insert((self.x, self.y + 1), true);
                chamber.insert((self.x + 1, self.y + 1), true);
            }
        }
        // for y in (1..=*chamber.keys().map(|(_, y)| y).max().unwrap()).rev() {
        //     print!("|");
        //     for x in 0..=6 {
        //         match chamber.get(&(x, y)) {
        //             Some(_) => print!("#"),
        //             None => print!("."),
        //         }
        //     }
        //     println!("|");
        // }
        // println!("+-------+");
    }
}

fn day_17(input: &str, height: usize) -> usize {
    use Shape::*;
    let mut chamber = HashMap::new();
    for x in 1..=7 {
        chamber.insert((x, 0), true);
    }
    let jet_dir = input.trim().chars().collect::<Vec<_>>();
    let mut moves = 0;
    let rocks = vec![
        Rock::new(Minus),
        Rock::new(Plus),
        Rock::new(Hook),
        Rock::new(Pipe),
        Rock::new(Square),
    ];
    for n in 0..height {
        let mut rock = rocks[n % rocks.len()].clone();
        rock.y = chamber.keys().map(|(_, y)| y).max().unwrap() + 4;
        loop {
            // print!("({}, {} )", rock.x, rock.y);
            let dir = jet_dir[moves % jet_dir.len()];
            match dir {
                '<' => rock.left(&chamber),
                '>' => rock.right(&chamber),
                _ => panic!("invalid dir"),
            };
            moves += 1;
            if !rock.down(&chamber) {
                break;
            }
        }
        rock.settle(&mut chamber);
    }
    *chamber.keys().map(|(_, y)| y).max().unwrap()
}

pub(crate) fn day_17_1(input: &str) -> usize {
    day_17(input, 2022)
}

pub(crate) fn day_17_2(input: &str) -> usize {
    day_17(input, 100000)
}

#[test]
fn test_day_17_1() {
    assert_eq!(day_17_1(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"), 3068);
}
