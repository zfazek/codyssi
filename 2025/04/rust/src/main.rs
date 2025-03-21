fn main() {
    let input = include_str!("../../input.txt");
    let n = input
        .chars()
        .filter(|&c| c.is_ascii_uppercase())
        .map(|x| x as usize)
        .map(|c| c - 'A' as usize + 1)
        .sum::<usize>();
    dbg!(n);
    let n = input
        .lines()
        .map(|line| {
            let vv = line.chars().collect::<Vec<_>>();
            let len = vv.len();
            let l10 = len / 10;
            let mut new_line = Vec::new();
            for i in 0..l10 {
                new_line.push(vv[i]);
            }
            let n = len - 2 * l10;
            let n = n.to_string();
            n.chars().for_each(|c| new_line.push(c));
            for i in len - l10..len {
                new_line.push(vv[i]);
            }
            sum_line(new_line)
        })
        .sum::<usize>();
    dbg!(n);
    let n = input
        .lines()
        .map(|line| {
            let vv = line.chars().collect::<Vec<_>>();
            let mut new_line = Vec::new();
            let mut c = vv[0];
            let mut s = 1;
            for i in 1..vv.len() {
                let c1 = vv[i];
                if c != c1 {
                    new_line.push(c);
                    let ss = s.to_string();
                    ss.chars().for_each(|c| new_line.push(c));
                    c = c1;
                    s = 1;
                } else {
                    s += 1;
                }
            }
            new_line.push(vv[vv.len() - 1]);
            let ss = s.to_string();
            ss.chars().for_each(|c| new_line.push(c));
            sum_line(new_line)
        })
        .sum::<usize>();
    dbg!(n);
}

fn sum_line(new_line: Vec<char>) -> usize {
    new_line
        .iter()
        .map(|&c| {
            if c.is_ascii_digit() {
                c as usize - '0' as usize
            } else {
                c as usize - 'A' as usize + 1
            }
        })
        .sum::<usize>()
}
