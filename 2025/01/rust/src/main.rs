fn main() {
    println!("Hello, world!");
    let input = include_str!("../../input.txt");
    let vv = input.lines().collect::<Vec<_>>();
    let mut dirs = vv[vv.len() - 1].chars().collect::<Vec<_>>();
    let mut d: i64 = vv[0].parse().unwrap();
    for i in 1..vv.len() - 1 {
        if dirs[i - 1] == '+' {
            d += vv[i].parse::<i64>().unwrap();
        } else {
            d -= vv[i].parse::<i64>().unwrap();
        }
    }
    println!("{}", d);
    dirs.reverse();
    let mut d: i64 = vv[0].parse().unwrap();
    for i in 1..vv.len() - 1 {
        if dirs[i - 1] == '+' {
            d += vv[i].parse::<i64>().unwrap();
        } else {
            d -= vv[i].parse::<i64>().unwrap();
        }
    }
    println!("{}", d);
    let mut d: i64 = vv[0].parse::<i64>().unwrap() * 10 + vv[1].parse::<i64>().unwrap();
    for i in 1..vv.len() / 2 {
        let j = i * 2;
        let n = vv[j].parse::<i64>().unwrap() * 10 + vv[j + 1].parse::<i64>().unwrap();
        if dirs[i - 1] == '+' {
            d += n;
        } else {
            d -= n;
        }
    }
    println!("{}", d);
}
