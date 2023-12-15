use advent2023::*;
fn main() {
    let things = parse(input!());
    //part 1
    let res = part1(things.clone());
    println!("Part 1: {}", res);
    //part 2
    let res = part2(things);
    println!("Part 2: {}", res);
}
type ParsedItem = String;
fn parse(input: &str) -> impl Iterator<Item = ParsedItem> + Clone + '_ {
    input.trim_end().split(',').map(|s| s.to_string())
}

fn hash(s: String) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}
fn part1<I>(things: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    things.map(hash).sum()
}

fn part2<I>(seq: I) -> usize
where
    I: Iterator<Item = ParsedItem>,
{
    let mut boxes: Vec<Vec<(String, u8)>> = vec![vec![]; 256];
    for ins in seq {
        let (label, num) = ins
            .split_once(|c| c == '-' || c == '=')
            .expect("no separator");
        let label = label.to_string();
        let h = hash(label.clone());
        match num {
            // -
            "" => {
                boxes[h].retain(|(l, _)| *l != label);
            }
            // =
            _ => {
                let num = num.parse::<u8>().expect("an integer");

                if let Some(i) =
                    boxes[h]
                        .iter()
                        .enumerate()
                        .find_map(|(i, (l, _))| if *l == label { Some(i) } else { None })
                {
                    // replace
                    boxes[h][i] = (label, num);
                } else {
                    // insert at the end
                    boxes[h].push((label, num));
                }
            }
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(boxi, b)| {
            b.iter()
                .enumerate()
                .map(|(lensi, (_, n))| (boxi + 1) * (lensi + 1) * *n as usize)
                .sum::<usize>()
        })
        .sum()
}

#[test]
fn test() {
    let things = parse("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
    //part 1
    let res = part1(things.clone());
    assert_eq!(res, 1320);
    //part 2
    let res = part2(things);
    assert_eq!(res, 145);
}
