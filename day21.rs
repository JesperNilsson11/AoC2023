use std::collections::HashMap;

fn task1() {
    let input = include_str!("input21.txt");
    let sum: i64;
    let mut map: Vec<Vec<char>> = Vec::new();

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
    for _ in 0..64 {
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
   
    println!("ans: {}", sum);
}

fn task2() {
    let input = include_str!("input21.txt");
    let mut sum: i64 = 0;
    let sum2: i64;
    let mut map: Vec<Vec<char>> = Vec::new();

    let mut pos: (i64, i64) = (0,0);
    for (y, line) in input.trim().lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                pos = (x as i64, y as i64);
            }
            row.push(c);
        }

        for _ in 0..4 {
            let len = row.len();
            for i in 0..len {
                row.push(row[i]);
            }
        }
        map.push(row);
    }

    let side = map.len();
    let half: i64 = side as i64 / 2;
    for _ in 0..4 {
        for i in 0..side as usize {
            map.push(map[i].clone());
        }
    }


    pos.0 += 131 * 2;
    pos.1 += 131 * 2;
    //println!("start pos: {:?}", pos);

    let mut queue = Vec::new();
    queue.push(pos);
    for _ in 0..327 {
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

    let mut scores = vec![vec![0; 5]; 5];

    for y in -2..=2 {
        let yy = pos.1 + y * side as i64;
        let yrange = (yy-half, yy+half);
        for x in -2..=2 {
            let xx = pos.0 + x * side as i64;
            let xrange = (xx-half, xx+half);

            let mut unique = HashMap::new();
            for q in &queue {
                if q.0 > xrange.1 || q.0 < xrange.0 ||
                   q.1 > yrange.1 || q.1 < yrange.0 {
                    continue;
                }
                unique.insert((q.0,q.1), 1);
            }
            scores[(y+2) as usize][(x+2) as usize] = unique.len() as i64;
        }
    }

    let range: i64 = (26501365-half as i64) / side as i64 - 1;
    let rrange: i64 = (26501365-half as i64) / side as i64;
    let mut start = 0;
    let mut non_start = 0;
    for y in -range..=range {
        let left = range - (y as i64).abs();
        non_start += left + 1;
        start += left;
    }

    sum += start * scores[2][2];
    sum += non_start * scores[2][3];

    sum += scores[0][2];
    sum += scores[4][2];
    sum += scores[2][0];
    sum += scores[2][4];

    sum += scores[0][1] * rrange;
    sum += scores[1][1] * range;

    sum += scores[0][3] * rrange;
    sum += scores[1][3] * range;

    sum += scores[3][0] * rrange;
    sum += scores[3][1] * range;

    sum += scores[4][3] * rrange;
    sum += scores[3][3] * range;

    let n = 202300;
    sum2 =  15427 * n * n + 15560 * n + 3921;

    println!("ans: {} {}", sum, sum2);
}

fn main() {
    task1();
    task2();
}
