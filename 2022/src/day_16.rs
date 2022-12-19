use std::collections::HashMap;

pub(crate) fn day_16_1(input: &str) -> usize {
    let path = vec!["AA"];
    let mut all_nodes = HashMap::new();
    let mut nodes_with_flow = HashMap::new();

    input.lines().for_each(|line| {
        if let Some((valve_part, tunnels_part)) = line.split_once(';') {
            let mut split = valve_part.split([' ', '=', ';']).skip(1);
            let valve = split.next().unwrap();
            let flow = split.last().unwrap().parse::<usize>().unwrap();
            let tunnels = tunnels_part
                .split([',', ' '])
                .filter(|x| !x.is_empty())
                .skip(4)
                .collect::<Vec<_>>();
            all_nodes.insert(valve, (flow, tunnels.clone()));
            if flow > 0 {
                nodes_with_flow.insert(valve, (flow, tunnels));
            }
        };
    });
    let mut current = "AA";
    let target = "JJ";
    // nodes_with_flow.keys().into_iter().for_each(|target| {
    // Find shortest path from current to target
    let mut visited = vec![];
    let mut paths = vec![];

    let available = all_nodes.keys();
    while visited.len() < available.len() {
        visited.push(current);
        paths.push(current);
        if current == target {
            println!("found. {:?}", path);
        } else {
            if let Some((_, tunnels)) = all_nodes.get(current) {
                if let Some(next) = tunnels.iter().filter(|x| !visited.contains(x)).next() {
                    current = next;
                } else {
                    paths.pop();
                }
            }
        }
    }
    // });
    // eprintln!("{:?}", nodes_with_flow);
    1
}

#[test]
fn test_day_16_1() {
    let input = String::from_utf8_lossy(include_bytes!("../test_input/day_16.txt"));
    assert_eq!(day_16_1(&input), 1651);
}
