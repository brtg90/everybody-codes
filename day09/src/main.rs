use std::collections::{HashMap, HashSet};

use itertools::izip;

use utils::read_lines;

fn main() {
    part_1();
    part_2();
    part_3();
}

fn part_1() {
    let sequences = parse("inputs/day09pt1.txt");
    let score = get_children_similarities(&sequences);
    println!("Part 1: {:?}", score)
}

fn part_2() {
    let sequences = parse("inputs/day09pt2.txt");
    let score = get_children_similarities(&sequences);
    println!("Part 2: {:?}", score);
}

fn part_3() {
    let sequences = parse("inputs/day09pt3.txt");
    let family_members = find_biggest_family(&sequences);
    let score: usize = family_members.iter().sum();
    println!("Part 3: {:?}", score);
}

fn compare_sequences(sequence_0: &[char], sequence_1: &[char]) -> usize {
    sequence_0.iter()
        .zip(sequence_1.iter())
        .filter(|(a, b)| a == b)
        .count()
}

fn get_children_similarities(sequences: &[(usize, Vec<char>)]) -> usize {
    let mut children: HashMap<usize, usize> = HashMap::new();
    for i in 0..sequences.len() {
        for j in 0..sequences.len() {
            for k in j + 1..sequences.len() {
                if i == j || i == k || children.contains_key(&j) || children.contains_key(&k) {
                    continue
                }
                if is_child_of(&sequences[i].1, &sequences[j].1, &sequences[k].1) {
                    let score = compare_sequences(&sequences[i].1, &sequences[j].1) *
                        compare_sequences(&sequences[i].1, &sequences[k].1);
                    children.insert(i, score);
                }
            }
        }
    }
    children.values().sum()
}

fn is_child_of(child: &[char], parent_0: &[char], parent_1: &[char]) -> bool {
    izip!(child, parent_0, parent_1)
        .all(|(ch, p0, p1)| ch == p0 || ch == p1)
}

fn find_biggest_family(sequences: &[(usize, Vec<char>)]) -> Vec<usize> {
    let mut children: HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 0..sequences.len() {
        for j in 0..sequences.len() {
            for k in j + 1..sequences.len() {
                if i == j || i == k {
                    continue
                }
                if is_child_of(&sequences[i].1, &sequences[j].1, &sequences[k].1) {
                    children.insert(i + 1, vec![sequences[j].0, sequences[k].0]);
                }
            }
        }
    }
    let mut lineages = children.keys()
        .map(|child| (*child, find_lineage(&children, *child, HashSet::new())))
        .collect::<Vec<(usize, HashSet<usize>)>>();

    // Updating one child's (grand-)parents can also lead to new values for already processed
    // children. Ensure that there is no non-updated value left
    let mut new_lineages = vec![];
    while new_lineages != lineages {
        new_lineages = lineages.clone();
        lineages = lineages.iter()
            .map(|(child, lineage)| {
                let mut lineage_set = lineages.iter()
                    .filter(|(_, lineage_2)| lineage.intersection(lineage_2).count() > 0)
                    .flat_map(|(_, lineage_2)| lineage_2.iter().copied().collect::<HashSet<usize>>())
                    .collect::<HashSet<usize>>();
                lineage_set.extend(lineage);
                (*child, lineage_set)
            })
            .collect::<Vec<(usize, HashSet<usize>)>>();
    }

    lineages.iter()
        .max_by_key(|(_, lineage)| lineage.len())
        .map(|(_, lineage)| lineage.iter().copied().collect::<Vec<usize>>())
        .unwrap()
}

fn find_lineage(children: &HashMap<usize, Vec<usize>>, child: usize, mut lineage: HashSet<usize>) -> HashSet<usize> {
    lineage.insert(child);
    if !children.contains_key(&child) {
        return lineage;
    }
    let new: HashSet<usize> = children.get(&child)
        .unwrap()
        .iter()
        .flat_map(|parent| find_lineage(children, *parent, lineage.clone()))
        .collect();
    lineage.extend(&new);
    lineage
}

fn parse(filename: &str) -> Vec<(usize, Vec<char>)> {
    read_lines(filename).iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut split = line.split(":");
            let index = split.next()
                .unwrap()
                .parse::<usize>()
                .expect("Failed to parse index");
            let sequence = split.next()
                .unwrap()
                .chars()
                .collect::<Vec<char>>();
            (index, sequence)
        })
        .collect::<Vec<(usize, Vec<char>)>>()
}