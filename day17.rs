use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Path {
    pos: (i64, i64),
    dir: i32,
    straight: i64,
    heat: i64,
}
/*
    dir
    0: up
    1: right
    2: down
    3: left
 */

 fn main() {
    let input = include_str!("input17.txt");
    let mut sum: i64 = i64::MAX;
    let mut map: Vec<Vec<i64>> = Vec::new();
    let mut hmap = HashMap::new();
    
    for line in input.trim().lines() {
        let mut row = Vec::new();
        for b in line.bytes() {
            row.push((b - b'0') as i64);
        }
        map.push(row);
    }

    let end: (i64, i64) = ((map[0].len() - 1) as i64, (map.len() - 1) as i64);

    let mut queue: Vec<Path> = vec![];
    queue.push(Path{pos: (0,0), dir: 1, straight: 0, heat: 0});
    queue.push(Path{pos: (0,0), dir: 2, straight: 0, heat: 0});
    let min_step = 4;
    let max_step = 10;
    while queue.len() > 0 {
        let mut p = queue.pop().unwrap();
        let factor: i64 = if p.straight < min_step { min_step }  else {1};

        let mut cont = false;
        for _ in 0..factor {
            match p.dir {
                0 => p.pos.1 -= 1,
                1 => p.pos.0 += 1,
                2 => p.pos.1 += 1,
                3 => p.pos.0 -= 1,
                _ => println!("Error"),
            }
            
            if p.pos.0 < 0 || p.pos.0 >= map[0].len() as i64
            || p.pos.1 < 0 || p.pos.1 >= map.len() as i64 {
                cont = true;
                break;
            }    
            p.heat += map[p.pos.1 as usize][p.pos.0 as usize];
        }
        if cont {
            continue;
        }

        if hmap.contains_key(&(p.pos.0, p.pos.1, p.straight, p.dir)) {
            let mut minheat = i64::MAX;

            for i in 0..p.straight {
                if hmap.contains_key(&(p.pos.0, p.pos.1, i, p.dir)) == true {
                    let temp = *hmap.get(&(p.pos.0, p.pos.1, i, p.dir)).unwrap();
                    if temp < minheat {
                        minheat = temp;
                    }
                }
            }
            let pp = hmap.get_mut(&(p.pos.0, p.pos.1, p.straight,p.dir)).unwrap();
            if minheat > *pp {
                minheat = *pp;
            }
                
            if minheat <= p.heat {
                continue;
            }
            *pp = p.heat;
        } else {
            hmap.insert((p.pos.0, p.pos.1, p.straight, p.dir), p.heat);
        }
            
        if p.pos == end {
            //println!("{} {}", p.heat, factor);
            if p.heat < sum {
                sum = p.heat;
                //println!("{}", sum);
            }
        }
        p.straight += 1 * factor;
        if p.straight < max_step {
            let p1 = p.clone();
            queue.push(p1);
        }
            
        match p.dir {
            0 | 2 => {
                p.straight = 0;
                let mut p1 = p.clone();
                p1.dir = 1;
                queue.push(p1);
                p.dir = 3;
                queue.push(p);
            },
            1 | 3 => {
                p.straight = 0;
                let mut p1 = p.clone();
                p1.dir = 0;
                queue.push(p1);
                p.dir = 2;
                queue.push(p);
            },
            _ => println!("Error"),
        }

        queue.sort_by(|a, b| (b.heat).cmp(&(a.heat)));
    }
        
    println!("ans: {}", sum);
}
    