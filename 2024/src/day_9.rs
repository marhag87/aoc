use itertools::Itertools;

fn main() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_9.txt"));
    let first = day_9_1(&input);
    println!("{:?}", first);
    let second = day_9_2(&input);
    println!("{:?}", second);
}

fn day_9_1(input: &str) -> usize {
    let mut id = 0;
    let mut start = 0;
    let mut disk = Vec::new();
    input
        .chars()
        .map(|character| character.to_digit(10).unwrap())
        .enumerate()
        .for_each(|(position, number)| {
            let file = position % 2 == 0;
            let end = start + number;
            for _position in start..end {
                if file {
                    disk.push(Some(id));
                } else {
                    disk.push(None);
                }
            }
            start = end;
            if file {
                id += 1;
            }
        });

    while disk
        .iter()
        .tuples()
        .any(|(first, second)| first.is_none() && second.is_some())
    {
        let last = disk.pop().unwrap();
        if let Some(last) = last {
            let first_free = disk
                .iter()
                .enumerate()
                .find_map(|(n, id)| if id.is_none() { Some(n) } else { None })
                .unwrap();
            disk[first_free] = Some(last);
        } else {
            continue;
        }
    }

    disk.iter()
        .enumerate()
        .filter_map(|(n, id)| id.map(|id| n * id as usize))
        .sum()
}

fn day_9_2(input: &str) -> usize {
    let mut id = 0;
    let mut disk = input
        .chars()
        .map(|character| character.to_digit(10).unwrap())
        .enumerate()
        .map(|(position, number)| {
            let file = position % 2 == 0;
            let section = if file {
                Section::File(File {
                    id,
                    size: number as usize,
                })
            } else {
                Section::Free(number as usize)
            };
            if file {
                id += 1;
            };
            section
        })
        .collect::<Vec<Section>>();
    let mut files = disk
        .iter()
        .filter_map(|section| match section {
            Section::File(file) => Some(*file),
            Section::Free(_) => None,
        })
        .collect::<Vec<File>>();
    files.sort_by(|a, b| b.id.cmp(&a.id));
    files.iter().for_each(|file| {
        let search = disk.clone();
        let mut moved = false;
        let mut new_free_size = 0;
        let mut moved_index = 0;
        search.iter().enumerate().find(|section| match section {
            (n, Section::Free(size)) => {
                if *size >= file.size && !moved {
                    new_free_size = size - file.size;
                    disk[*n] = Section::File(*file);
                    moved_index = *n;
                    moved = true;
                }
                false
            }
            (n, Section::File(disk_file)) => {
                if file == disk_file && moved {
                    disk[*n] = Section::Free(file.size);
                    true
                } else {
                    file == disk_file
                }
            }
        });
        if moved && new_free_size > 0 {
            disk.insert(moved_index + 1, Section::Free(new_free_size));
        }
    });
    let mut n = 0;
    disk.iter()
        .map(|section| match section {
            Section::Free(size) => {
                n += size;
                0
            }
            Section::File(file) => {
                let sum = (n..(n + file.size)).map(|m| m * file.id).sum();
                n += file.size;
                sum
            }
        })
        .sum()
}

#[derive(Copy, Clone, PartialEq)]
enum Section {
    File(File),
    Free(usize),
}

#[derive(Copy, Clone, PartialEq)]
struct File {
    id: usize,
    size: usize,
}

#[test]
fn test_day_9_1() {
    let input = r#"2333133121414131402"#;
    assert_eq!(day_9_1(input), 1928);
}

#[test]
fn test_day_9_2() {
    let input = r#"2333133121414131402"#;
    assert_eq!(day_9_2(input), 2858);
}

#[ignore]
#[test]
fn test_day_9_1_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_9.txt"));
    assert_eq!(day_9_1(&input), 6241633730082);
}

#[ignore]
#[test]
fn test_day_9_2_answer() {
    let input = String::from_utf8_lossy(include_bytes!("../input/day_9.txt"));
    assert_eq!(day_9_2(&input), 6265268809555);
}
