use regex::Regex;

fn contains(one: u64, two: u64, three: u64, four: u64) -> bool {
    (one >= three && two <= four) || (three >= one && four <= two)
}

fn overlaps(one: u64, two: u64, three: u64, four: u64) -> bool {
    (one..=two)
        .into_iter()
        .any(|num| (three..=four).contains(&num))
}

pub(crate) fn day_4_1(input: String) -> u64 {
    let re = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").expect("should compile");
    re.captures_iter(&input)
        .map(|cap| {
            contains(
                cap[1].parse::<u64>().expect("should be a number"),
                cap[2].parse::<u64>().expect("should be a number"),
                cap[3].parse::<u64>().expect("should be a number"),
                cap[4].parse::<u64>().expect("should be a number"),
            ) as u64
        })
        .sum::<u64>()
}

pub(crate) fn day_4_2(input: String) -> u64 {
    let re = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").expect("should compile");
    re.captures_iter(&input)
        .map(|cap| {
            overlaps(
                cap[1].parse::<u64>().expect("should be a number"),
                cap[2].parse::<u64>().expect("should be a number"),
                cap[3].parse::<u64>().expect("should be a number"),
                cap[4].parse::<u64>().expect("should be a number"),
            ) as u64
        })
        .sum()
}

#[test]
fn test_contains() {
    assert!(!contains(2, 4, 6, 8));
    assert!(!contains(2, 3, 4, 5));
    assert!(!contains(5, 7, 7, 9));
    assert!(contains(2, 8, 3, 7));
    assert!(contains(6, 6, 4, 6));
    assert!(!contains(2, 6, 4, 8));
}

#[test]
fn test_overlaps() {
    assert!(!overlaps(2, 4, 6, 8));
    assert!(!overlaps(2, 3, 4, 5));
    assert!(overlaps(5, 7, 7, 9));
    assert!(overlaps(2, 8, 3, 7));
    assert!(overlaps(6, 6, 4, 6));
    assert!(overlaps(2, 6, 4, 8));
}
