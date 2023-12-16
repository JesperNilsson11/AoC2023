use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Beam {
    pos: (i64, i64),
    dir: i64,
}

fn calc(map: &Vec<Vec<char>>, start: Beam) -> i64 {
    let mut sum: i64 = 0;
    let mut hmap = HashMap::new();
    let mut queue: Vec<Beam> = Vec::new();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
   
    queue.push(start);

    while queue.len() > 0 {
        let mut b = queue.pop().unwrap();
        
        if hmap.contains_key(&b) == true {
            continue;
        }
        hmap.insert(b.clone(), 1);

        match b.dir {
            0 => b.pos.1 -= 1,
            1 => b.pos.0 += 1,
            2 => b.pos.1 += 1,
            3 => b.pos.0 -= 1,
            _ => println!("Error"),
        }
        
        if b.pos.0 < 0 || b.pos.0 >= map[0].len() as i64
        || b.pos.1 < 0 || b.pos.1 >= map.len() as i64 {
            continue;
        }
        if visited[b.pos.1 as usize][b.pos.0 as usize] == false {
            sum += 1;
        }
        visited[b.pos.1 as usize][b.pos.0 as usize] = true;

        match map[b.pos.1 as usize][b.pos.0 as usize] {
            '/' => {
                match b.dir {
                    0 => b.dir = 1,
                    1 => b.dir = 0,
                    2 => b.dir = 3,
                    3 => b.dir = 2,
                    _ => println!("Error dir"),
                }
            },
            '\\' => {
                match b.dir {
                    0 => b.dir = 3,
                    1 => b.dir = 2,
                    2 => b.dir = 1,
                    3 => b.dir = 0,
                    _ => println!("Error dir"),
                }
            },
            '|' => {
                match b.dir {
                    0 => (),
                    1 | 3 => {
                        let mut b1 = b.clone();
                        b1.dir = 0;
                        queue.push(b1);
                        b.dir = 2;
                    },
                    2 => (),
                    _ => println!("Error dir"),
                }
            },
            '-' => {
                match b.dir {
                    1 | 3 => (),
                    0 | 2 => {
                        let mut b1 = b.clone();
                        b1.dir = 1;
                        queue.push(b1);
                        b.dir = 3;
                    },
                    _ => println!("Error dir"),
                }
            },
            '.' => (),
            _ => println!("Error"),
        }

        queue.push(b);
    }

    return sum;
}

/*
    dir
    0: up
    1: right
    2: down
    3: left
 */

fn main() {
    let input = include_str!("input16.txt");
    let mut sum: i64 = 0;
    let mut sum2: i64 = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    
    for line in input.trim().lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }

    sum = calc(&map, Beam{pos: (-1, 0), dir: 1});

    for x in 0..map[0].len() as i64 {
        let t = calc(&map, Beam{pos: (x, -1), dir: 2});
        if t > sum2 {
            sum2 = t;
        }

        let t = calc(&map, Beam{pos: (x, map.len() as i64), dir: 0});
        if t > sum2 {
            sum2 = t;
        }
    }
    for y in 0..map.len()  {
        let t = calc(&map, Beam{pos: (-1, y as i64), dir: 1});
        if t > sum2 {
            sum2 = t;
        }

        let t = calc(&map, Beam{pos: (map[y].len() as i64, y as i64), dir: 3});
        if t > sum2 {
            sum2 = t;
        }
    }

    println!("ans: {} {}", sum, sum2);
}
