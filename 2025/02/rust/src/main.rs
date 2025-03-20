fn main() {
    let input = include_str!("../../input.txt");
    let mut vv = input.lines().collect::<Vec<_>>();
    let a = vv[0]
        .split_ascii_whitespace()
        .nth(3)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let b = vv[1]
        .split_ascii_whitespace()
        .nth(3)
        .unwrap()
        .parse::<i64>()
        .unwrap();
    for _ in 0..4 {
        vv.remove(0);
    }
    vv.sort();
    let v = vv[vv.len() / 2].parse::<i64>().unwrap();
    let r = v * v * v * b + a;
    dbg!(r);
    let v = vv
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .filter(|&x| x % 2 == 0)
        .sum::<i64>();
    let r = v * v * v * b + a;
    dbg!(r);
    let mut r = vv
        .iter()
        .map(|&x| (x.parse::<i64>().unwrap(), x.parse::<i64>().unwrap()))
        .map(|(x, v)| (v * v * v * b + a, x))
        .filter(|(x, _)| *x <= 15000000000000)
        .collect::<Vec<_>>();
    r.sort();
    let r = r[r.len() - 1].1;
    dbg!(r);
}
