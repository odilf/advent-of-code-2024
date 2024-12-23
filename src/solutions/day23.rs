use std::{
    collections::{HashMap, HashSet},
    str::from_utf8,
};

use itertools::Itertools as _;

type Node<'a> = &'a [u8];

fn parse(input: &str) -> (HashSet<Node>, HashMap<Node, HashSet<Node>>) {
    let mut connections = HashMap::new();
    let mut nodes = HashSet::new();

    for line in input.lines() {
        let mut cities = line.split('-');
        let a = cities.next().unwrap().as_bytes();
        let b = cities.next().unwrap().as_bytes();

        connections.entry(a).or_insert(HashSet::new()).insert(b);
        connections.entry(b).or_insert(HashSet::new()).insert(a);
        nodes.insert(a);
        nodes.insert(b);
    }

    (nodes, connections)
}

#[elvish::solution(day = 23, example = 7)]
fn part1(input: &str) -> usize {
    let (nodes, connections) = parse(input);

    let mut trios = HashSet::new();
    for node in &nodes {
        if !node.starts_with(&[b't']) {
            continue;
        }

        let neighbors = &connections[node];
        for a in neighbors {
            for b in neighbors {
                if connections[a].contains(b) {
                    let mut trio = [node, a, b];
                    trio.sort();
                    trios.insert(trio);
                }
            }
        }
    }

    trios.len()
}

#[elvish::solution(day = 23)]
fn part2(input: &str) -> String {
    let (nodes, connections) = parse(input);

    let mut max_subset = vec![];
    for node in nodes.iter() {
        let mut queue = connections[node]
            .iter()
            .map(|&node| vec![node])
            .collect::<Vec<_>>();

        let mut visited = HashSet::new();

        while let Some(mut connected_nodes) = queue.pop() {
            connected_nodes.sort();
            if !visited.insert(connected_nodes.clone()) {
                continue;
            }

            if connected_nodes.len() > max_subset.len() {
                max_subset = connected_nodes.clone();
            }

            let intersection = connected_nodes[1..]
                .iter()
                .map(|&node| connections[node].clone())
                .fold(connections[connected_nodes[0]].clone(), |a, b| {
                    a.intersection(&b).map(|&v| v).collect()
                });

            for next_node in intersection {
                let mut next_nodes = connected_nodes.clone();
                next_nodes.push(next_node);
                queue.push(next_nodes);
            }
        }
    }

    max_subset.sort();
    max_subset
        .into_iter()
        .map(|node| from_utf8(node).unwrap())
        .join(",")
}

#[test]
fn example_part2() {
    assert_eq!(part2(EXAMPLE_PART2).as_str(), "co,de,ka,ta");
}

elvish::example!(
    "
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    "
);
