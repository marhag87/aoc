pub(crate) fn day_1(input: &str) -> (u64, u64) {
    let mut nums: Vec<u64> = input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split('\n')
                .map(|num| num.parse::<u64>().unwrap_or_default())
                .sum::<u64>()
        })
        .collect();
    nums.sort();
    let max = nums.iter().max().expect("should have a max");
    let top_three = nums.iter().rev().take(3).sum::<u64>();
    (*max, top_three)
}
