use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../../input.txt");
    let n = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" ").unwrap();
            let (a1, a2) = a.split_once("-").unwrap();
            let (b1, b2) = b.split_once("-").unwrap();
            let a1 = a1.parse::<usize>().unwrap();
            let a2 = a2.parse::<usize>().unwrap();
            let b1 = b1.parse::<usize>().unwrap();
            let b2 = b2.parse::<usize>().unwrap();
            let mut s = BTreeSet::new();
            for i in a1..=a2 {
                s.insert(i);
            }
            for i in b1..=b2 {
                s.insert(i);
            }
            (a2 - a1 + 1 + b2 - b1 + 1, s.len())
        })
        .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1))
        .unwrap();
    dbg!(n);
    let n = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" ").unwrap();
            let (a1, a2) = a.split_once("-").unwrap();
            let (b1, b2) = b.split_once("-").unwrap();
            let a1 = a1.parse::<usize>().unwrap();
            let a2 = a2.parse::<usize>().unwrap();
            let b1 = b1.parse::<usize>().unwrap();
            let b2 = b2.parse::<usize>().unwrap();
            let mut s = BTreeSet::new();
            for i in a1..=a2 {
                s.insert(i);
            }
            for i in b1..=b2 {
                s.insert(i);
            }
            s
        })
        .collect::<Vec<_>>();
    let mut count = 0;
    for i in 0..n.len() - 1 {
        let v1 = n[i].clone();
        let v2 = n[i + 1].clone();
        let v = v1.union(&v2).cloned().collect::<BTreeSet<_>>();
        if v.len() > count {
            count = v.len();
        }
    }
    dbg!(count);
}
