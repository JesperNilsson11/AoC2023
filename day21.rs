use std::collections::HashMap;

fn task1() {
    let input = include_str!("input21.txt");
    let mut sum: i64 = 0;
    let mut sum2: i64 = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    //let mut hmap = HashMap::new();

    let mut pos: (i64, i64) = (0,0);
    for (y, line) in input.trim().lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                pos = (x as i64, y as i64);
            }
            row.push(c);
        }
        map.push(row);
    }

    let mut queue = Vec::new();
    queue.push(pos);
    let mut prev = 0;
    for i in 0..129 {
        let mut visited = HashMap::new();
        let mut newqueue = Vec::new();

        while queue.len() > 0 {
            let q = queue.pop().unwrap();

            if visited.contains_key(&q) {
                continue;
            }
            visited.insert(q, 1);
            for delta in [(0,1), (0,-1), (1,0), (-1,0)] as [(i64, i64); 4] {
                let q2 = (q.0+delta.0, q.1+delta.1);
                
                if q2.0 < 0 || q2.0 >= map[0].len() as i64 || q2.1 < 0 || q2.1 >= map.len() as i64 {
                    continue;
                }
                if visited.contains_key(&q2) {
                    continue;
                }

                if map[q2.1 as usize][q2.0 as usize] != '#' {
                    newqueue.push(q2);
                }
            }
        }

        queue = newqueue;
    }

    let mut unique = HashMap::new();
    for q in queue {
        unique.insert(q, 1);
    }
    sum = unique.len() as i64;
   
    println!("ans: {} {}", sum, sum2);
}

fn calc(map: &Vec<Vec<char>>, pos: (i64, i64), steps: i64) -> i64 {
    if steps > 300 {
        return 7697;
    }
    if steps < 0 {
        return 0;
    }
    if steps == 0 {
        println!("steps 0");
        return 1;
    }
    let mut queue = Vec::new();
    queue.push(pos);
    for i in 0..steps {
        let mut visited = HashMap::new();
        let mut newqueue = Vec::new();

        while queue.len() > 0 {
            let q = queue.pop().unwrap();

            if visited.contains_key(&q) {
                continue;
            }
            visited.insert(q, 1);
            for delta in [(0,1), (0,-1), (1,0), (-1,0)] as [(i64, i64); 4] {
                let q2 = (q.0+delta.0, q.1+delta.1);
                
                if q2.0 < 0 || q2.0 >= map[0].len() as i64 || q2.1 < 0 || q2.1 >= map.len() as i64 {
                    continue;
                }
                if visited.contains_key(&q2) {
                    continue;
                }

                if map[q2.1 as usize][q2.0 as usize] != '#' {
                    newqueue.push(q2);
                }
            }
        }

        queue = newqueue;
    }

    let mut unique = HashMap::new();
    for q in queue {
        unique.insert(q, 1);
    }
    return unique.len() as i64;
}

fn task2() {
    let input = include_str!("input21.txt");
    let mut sum: i64 = 0;
    let mut sum2: i64 = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    //let mut hmap = HashMap::new();

    let mut pos: (i64, i64) = (0,0);
    for (y, line) in input.trim().lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                pos = (x as i64, y as i64);
            }
            row.push(c);
        }
        map.push(row);
    }

    let mut min = -1;
    while min > -26501365 {
        min -= 131;
    }
    let mut max = 1;
    while min < 26501365 {
        min += 131;
    }
    let times = (26501365-65) / 131;
    for y in -times..=times {
        println!("{}", y);
        for x in -times..=times {
            let steps = 26501365 - ((x as i64).abs() * 131 + (y as i64).abs() * 131);
            let mut pos = (0, 0);
            if pos.0 > 0 {
                pos.0 = 0;
            } else {
                pos.0 = 130;
            }
            if pos.1 > 0 {
                pos.1 = 0;
            } else {
                pos.1 += 130;
            }
            sum += calc(&map, pos, steps);
        }
    }
   
    println!("ans: {} {}", sum, sum2);
}

fn main() {
    task1();
    task2();
}
