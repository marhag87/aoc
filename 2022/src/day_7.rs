use std::collections::HashMap;

pub(crate) fn day_7(input: &str) -> (usize, usize) {
    let mut dir_stack = vec![];
    let mut dir_sizes = HashMap::new();
    input.lines().for_each(|line| {
        if line.starts_with("$ cd") {
            // Put the dir on the stack
            match line.split_ascii_whitespace().rev().next() {
                Some("..") => {
                    let _ = dir_stack.pop();
                }
                Some("/") => dir_stack = vec!["/".to_string()],
                Some(dir) => {
                    let path = dir_stack.join("/");
                    let dir = format!("{}/{}", path, dir);
                    dir_stack.push(dir);
                }
                None => {
                    panic!("unexpected command")
                }
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
            // Do nothing
        } else {
            // File with size, add to all dirs on stack
            let size = line
                .split_ascii_whitespace()
                .next()
                .expect("should have a value")
                .parse::<usize>()
                .expect("should be a number");
            dir_stack.iter().for_each(|dir| {
                if let Some(val) = dir_sizes.get(dir) {
                    dir_sizes.insert(dir.clone(), val + size);
                } else {
                    dir_sizes.insert(dir.clone(), size);
                };
            })
        }
    });
    let sum_of_small_dirs = dir_sizes
        .iter()
        .filter_map(|(_, size)| if size <= &100000 { Some(size) } else { None })
        .sum();

    // Part 2
    let needed_space = 30000000 - (70000000 - dir_sizes.get("/").expect("root dir should exist"));
    let smallest_dir = dir_sizes
        .iter()
        .filter_map(|(_, size)| {
            if size >= &needed_space {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .expect("should have a min value");

    (sum_of_small_dirs, *smallest_dir)
}

#[test]
fn test_day_7_1() {
    let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
    let (part_1, _) = day_7(input);
    assert_eq!(part_1, 95437);
}

#[test]
fn test_day_7_2() {
    let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
    let (_, part_2) = day_7(input);
    assert_eq!(part_2, 24933642);
}
