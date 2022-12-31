use std::ops::Index;

const _INPUT_EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
const _INPUT_PART1: &str = "abccccaaaaaaaaaaaaaccaaaaaaaacccccccccaaaaaaaaccccccccaaacaaacccccccaaaaaaccccccccccccccccccccccaaaacccccccccccacccccccccccccccccccccccccccccccccccccccccccccccaaaa
abccccaaaaacaaaaaaccccaaaaaaccccccccccaaaaaaacccccccccaaaaaaacccccaaaaaaaaaacccccccccccccccccccaaaaaacccccccccaaaaaaaaccccccccccccccccccccccccccccccccccccccccaaaaa
abcccaaaaaccaaaaaaccccaaaaaaccccccaacccaaaaaacccccccccaaaaaacccaaaaaaaaaaaaaaacaaccacccccccccccaaaaaaccccccccccaaaaaacccccccccccccccccccccccccccccccccccccccccaaaaa
abccccccaaccaaaaaaccaaaaaaaaccccccaaacaaaacaaacccccccaaaaaaaaccaaaaaaaaaaaaaaacaaaaacccccccccccccaaccccccccccccaaaaaaccccccccccccccccccccccccccccacccccccccccaaaaaa
abccccccccccaaccaaccaaaaccaacccccccaaaaaaaccccccccccaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaacccccccaaaaccccccccccccccccaaaaaaaacccccccccccccccccccccccccccaaccccccccccccccaa
abcccccccaaaaacccaaaaaaaacccaaccccaaaaaaccccccccccccaaaaaaaaaaaaaaaacaaaaaaaccaaaaaaccccccaaaaaccccccccccccccaaaaaaaaaaccaccccccccccccccccccccccccaccccccccccccccca
abcccccccaaaaacccaaaaaaaaccaaaaccaaaaaaaaccccccccccccccaaacaaaaaaaaacaaaaaacccccaaaacccccaaaaaaccccccccccccccaaaaaaaaaaaaacccccccccccccccllllllccccdccccccccccccccc
abccccccaaaaaacccccaaaaccccaaaaacaaaaaaaaccccccccccccccaaacccccaaaccccaaaaaacccaaccccccccaaaaaacccccccccccccccccaaaaaaaaaacccccccccccccklllllllllcddddccaccaaaccccc
abccccccaaaaaacccccaaaaaaaaaaaaaaaccaaccccccaacaacccccccaaccccccccccccaaacaacccccccccccccaaaaaacccccccccccccccccaaaaaaaaaacccccccccccckklllppllllcddddddddaaaaccccc
abccccccaaaaaaccccaaacaaaaaaaaaaaaccaaccccccaaaaaccccccccccccccccccccccccccccccccccccccccccaaccccccaaccccccccccccaacaaaaaaaccccccccccckklpppppplllmdddddddddacccccc
abccccccccaaacccccaacccaccaaaaaaccccccccccccaaaaaaccccccccccccccccccccccccccccccccccccccccccccccccaaaaccccccccccccaaaaaaaaaaccccccccckkkkppppppplmmmmmmddddddaacccc
abccccaaacaaacccccccccccccaaaaaaccccccccccccaaaaaacccccccccccccccccaaaccccccccccccccccccccccccccccaaaaccccccccccccaaaaaaaaaaccccccccckkkppppuppppmmmmmmmmddeeeacccc
abccccaaaaaaacccccccccccccaaaaaacccaccccccccaaaaaacccccccccccccccccaaaacccccccccccccccccccccaaacccaaaacccccccccccaaaacaaaccccccccccckkkpppuuuuuppqqmmmmmmmmeeeacccc
abcccccaaaaaaccccccccccccaaaaaaaacaaccccccccccaaaccccccccccccccccccaaaaccccccccccccccccccccaaaaccccccccccccccccccaaaaaaaacccccccccckkkkpppuuuuupqqqqqqqmmmmeeeccccc
abcccccaaaaaaaacccccccccccaccccaaaaacccccccccccccccccccccccccccccccaaaccccccccccccccaaaccccaaaacccccccccccccaaccaaaaaaaaccccccccckkkkkrrpuuuxuuuqqqqqqqqmmmmeeccccc
abccccaaaaaaaaaccccccccccccccaaaaaacccccccacaacccccccccccccccccccccccccccccccccccccaaaaaacccaaaccccccccccaaaaccaaaaaaacccccccccckkkkrrrrruuuxxuvvvvvvqqqqnnneeccccc
abcccaaaaaaaaaaccccccccccccccaaaaaaaacccccaaaaacccccccccccccccaaaaaccccccccccccccccaaaaaaccccccccccccccccaaaaaaaaaaaaacccccccccjjjkrrrrruuuxxxxvvvvvvvqqqnnneeccccc
abcaaaaacaaacccccccccccccccccaaaaaaaacccccaaaaaccaacccccccccccaaaaaccccccccccccccccaaaaaccccccccccccccccccaaaaaccaaaaaacccccccjjjrrrrruuuuuxxxyvyyyvvvqqqnneeeccccc
abcaaaaacaaaccaaccccccccccccccccaacccccccaaaaaaaaaaaccccccccccaaaaaaccccccccccccccccaaaaaccccccccccccccccaaaaacccaaaaaaaacaaacjjjrrrtttuuxxxxxyyyyyvvvqqnnneeeccccc
abaaaaaccaacccaaaccaacccaaccccccaccccccccaaaaaaaaaacccccccccccaaaaaaccccccccccccccccaacaacccccccccccccccccccaacccaaccccaaaaaacjjjrrrtttxxxxxxxyyyyyvvvrrnnneeeccccc
SbaaaaacccccccaaaaaaaccaaaacccccccccccccccaaaaaaaaacccccccccccaaaaaaccccccccccccccccccccccccccccccccccccccccccccccaacccaaaaaacjjjrrrtttxxxEzzzzyyyvvvrrnnneeecccccc
abcaaaaacccccccaaaaaaccaaaacccccccccccccccaaaaaaaaacccccccccccccaaccccccccccccccccccccccccccccaaccccccccccaaccccacaaaacaaaaaaajjjrrrtttxxxxxyyyyyvvvrrrnnnfffcccccc
abcaacccccccaaaaaaaacccaaaaccccccccccccccccaaaaaaaaaaccccccccccccccccccccccccccccccccccccccaaaaaccccccccccaaccccaaaaaaaaaaaaaajjjqqqttttxxxxyyyyyyvvrrrnnnfffcccccc
abccccccccccaaaaaaaaaccccccccccccccccccccaaaaaaaaaaaaacccccccccccccccccaacccccccccccccccccccaaaaaccccccaacaaaaaccaaaacaaaaaaaacjjjqqqqttttxxyywyyyywvrrnnnfffcccccc
abccccccccccaaaaaaaaaacccccccccccccccccccaaaaaaaaacaaacccccccccccccaaacaacccccccccccccccccccaaaaaccccccaaaaaaaaccaaaaccccaaacccjjjjqqqqtttxwywwwyywwwrrnnnfffcccccc
abcccccccccccccaaaaaaacccccccccccccccccccaaaaaaaaaaaaaaaacccccccccccaaaaaccccccccccccccccccaaaaacccaaccccaaaaccccaacaacccaaaccccjjjiqqqtttwwywwwwwwwwrrroofffcccccc
abcccccccccccccaaaccccccccccccccccccccccaaaaaaaaaaaaaaaaaccccccccccccaaaaaacccccccccccccccccccaaacaaaccccaaaaaccccccccccccccccccciiiiqqqttwwwwwswwwwrrrroofffcccccc
abcccccccccccccaaccccccccccccaaaacccccccaaaaaaaaccaaaaacccccccccccccaaaaaaacccccccccccccccccccaaaaaaacccaaacaacccccaaaaacccccccccciiiqqqttwwwwsssssrrrrroofffaccccc
abcccccccccccccccccccccccccccaaaaccccccccacaaacccaaaaaaccccccaaccccaaaaaaccccccccaacaaccccccccaaaaaaccccaaaacacccccaaaaacccccccccciiiqqqtsswsssssssrrrrooofffaccccc
abcccccccccccccccccccccccccccaaaaccccccccccaaaccaaaaaaaccccccaaaaccaacaaaccccccccaaaaacccccccccaaaaaaaaccaaacacccccaaaaaacccccccccciiqqqssssssspposrrroooofffaccccc
abccccaaacccccccccccccccccccccaaacccccccccccccccaaacaaaccccaaaaaacccccaaaccccccccaaaaaacccccccaaaaaaaaaaaaaaaaaccccaaaaaaccccccaccciiiqqpsssssppppooooooogffaaccccc
abccccaaaaaacccaaaccccccccccccccccccccccccccccccccccccaccccaaaaacccccccccccccccccaaaaaaccccccaaaaaaaaaaaaaaaaaaccccaaaaaacccaaaaccciiiqqppppppppppoooooogggfaaacccc
abcccaaaaaaacccaaaccccccccccccccccccccccccccccccccccccccccccaaaaaccccccccccccccccaaaaaaccccccaaacaaaccccaaaaaacccccccaacccccaaaaaacciiipppppppphgggggggggggaaaacccc
abccaaaaaaaacccaaacaaacccccccccccccccccccccaacccccccccccccccaacaacccccaacccccccccccaaacccccccccccaaacccccaaaaacccccccccccccccaaaaacciiihppppphhhhgggggggggaaccccccc
abccaaaaaaacaaaaaaaaaacccccccccccccccccccccaaaccccccacccccccccccccccccaaccccccccccccccccccccccccaaaaccccaaaaaaccccccccccccccaaaaacccciihhhhhhhhhhgggggggccaaccccccc
abccccaaaaaaaaaaaaaaacccccccccccccccccccaaaaaaaaccccaaacaaaccccccccccaaaaccaaccccccccaacaacccccaaaaaaacccaacccccccccccccccccaacaaccccchhhhhhhhhaaaacccccccccccccccc
abccccaaaaaacaaaaaaaccccccccccccccccccccaaaaaaaaccccaaaaaaaccccccccccaaaaaaaacaccccccaaaaaccccccaaaaacccccccccccccccccccccccccccccccccchhhhhhacaaaaaccccccccccccccc
abccccaaccccccaaaaaacccccccccccccaaccccccaaaaaacccccaaaaaaccccccaaaaaaaaaaaaaaaccccccaaaaaacccaaaaaaacccccccccccccccccccccccccccccccccccccaaaaccaaacccccccccccaaaca
abccccccccccccaaaaaaaccccccccccccaaccccccaaaaaacccaaaaaaaaccccccaaaaaaaaaaaaaacccccccaaaaaacccaaaaaaaaccccccaaacccccccccccccccccccccccccccaaaaccccccccccccccccaaaaa
abccaaacccccccaaacaaacccccccccaaaaaaaacccaaaaaacccaaaaaaaaacccccaaaaaaaaaaaaaacccccccaaaaaccccaaaaaaaaccccccaaaaccccccccccccccccccccccccccaaaccccccccccccccccccaaaa
abcaaaacccccccaaccccccccccccccaaaaaaaacccaaccaacccaaaaaaaaaaccccccccaaaaaaacaacccccccccaaaccccccaaacaaccccccaaaacccccccccccccccccccccccccccccccccccccccccccccaaaaaa";

struct Map {
    width: i32,
    height: i32,
    cells: Vec<Vec<i32>>,
    distances: Vec<Vec<i32>>,
    start: (usize, usize),
    finish: (usize, usize),
}

fn parse_input(input: &str) -> Map {
    let mut cells: Vec<Vec<i32>> = input.split("\n").map(|row| row.chars().map(|c| (c as i32) - ('a' as i32)).collect()).collect();
    let height = cells.len();
    let width = cells[0].len();

    let start = cells
        .iter()
        .enumerate()
        .map(|(y, row)| (row.iter().position(|c| *c == ('S' as i32 - 'a' as i32)).unwrap_or(9999999), y))
        .min()
        .unwrap();
    let finish = cells
        .iter()
        .enumerate()
        .map(|(y, row)| (row.iter().position(|c| *c == ('E' as i32 - 'a' as i32)).unwrap_or(9999999), y))
        .min()
        .unwrap();

    cells[start.1][start.0] = 0;
    cells[finish.1][finish.0] = 26;

    Map {
        width: width as i32,
        height: height as i32,
        cells,
        distances: vec![vec![99999999; width]; height],
        start,
        finish,
    }
}

fn dijsktra(map: &mut Map) {
    let mut stack = Vec::new();
    stack.push(map.finish);
    map.distances[map.finish.1][map.finish.0] = 0;
    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();
        let distance = map.distances[y][x] + 1;
        for dir in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let from = (x as i32 + dir.0, y as i32 + dir.1);
            if from.0 < 0 || from.1 < 0 || from.0 >= map.width || from.1 >= map.height {
                continue;
            }
            let fx = from.0 as usize;
            let fy = from.1 as usize;

            if (map.cells[y][x] - map.cells[fy][fx]) <= 1 && map.distances[fy][fx] > distance {
                map.distances[fy][fx] = distance;
                stack.push((fx, fy));
            }
        }
    }
}

fn part1() {
    let mut map = parse_input(_INPUT_PART1);
    println!("start: {:?}, finish: {:?}", map.start, map.finish);
    dijsktra(&mut map);
    for row in map.distances.iter() {
        for dist in row {
            print!("[{:3}]", std::cmp::min(999, *dist));
        }
        println!("");
    }
    println!("answer: {}", map.distances[map.start.1][map.start.0]);
}


fn part2_dijsktra(map: &mut Map) {
    let mut stack = Vec::new();
    stack.push(map.finish);
    map.distances[map.finish.1][map.finish.0] = 0;
    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();
        let distance = map.distances[y][x] + std::cmp::min(1, map.cells[y][x]);
        for dir in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let from = (x as i32 + dir.0, y as i32 + dir.1);
            if from.0 < 0 || from.1 < 0 || from.0 >= map.width || from.1 >= map.height {
                continue;
            }
            let fx = from.0 as usize;
            let fy = from.1 as usize;

            if (map.cells[y][x] - map.cells[fy][fx]) <= 1 && map.distances[fy][fx] > distance {
                map.distances[fy][fx] = distance;
                stack.push((fx, fy));
            }
        }
    }
}

fn part2() {
    let mut map = parse_input(_INPUT_PART1);
    println!("start: {:?}, finish: {:?}", map.start, map.finish);
    part2_dijsktra(&mut map);
    for row in map.distances.iter() {
        for dist in row {
            print!("[{:3}]", std::cmp::min(999, *dist));
        }
        println!("");
    }
    println!("answer: {}", map.distances[map.start.1][map.start.0]);}

fn main() {
    println!("part1");
    part1();
    println!("part2");
    part2();
}
