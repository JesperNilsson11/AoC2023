use std::collections::HashMap;
use std::cmp::min;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Brick {
    x: i32,
    y: i32,
    z: i32,
    x2: i32,
    y2: i32,
    z2: i32,
    id: usize,
}

fn calc(parents: &HashMap<usize, HashMap<usize, bool>>, children: &HashMap<usize, HashMap<usize, bool>>,
    moved: &mut HashMap<usize, bool>, id: usize) -> i64 {
    let mut sum = 0;
    moved.insert(id, true);

    let p = parents.get(&id);
    for pp in p {
        for (k, v) in pp {
            let chs = children.get(&k).unwrap();
            let mut should_move = true;
            for (cid, _) in chs {
                if moved.contains_key(&cid) == false {
                    should_move = false;
                    break;
                }
            }

            if should_move {
                sum += 1;
                sum += calc(parents, children, moved, *k);
            }
        }
    }

    return sum;
}

fn main() {
    let input = include_str!("input22.txt");
    let mut sum: i64 = 0;
    let mut sum2: i64 = 0;
    //let mut map: Vec<Vec<char>> = Vec::new();
    let mut hmap = HashMap::new();
    let mut bricks = Vec::new();

    for (i, line) in input.trim().lines().enumerate() {
        let mut b = Brick{x: 0, y: 0, z: 0, x2: 0, y2: 0, z2: 0, id: i};
        let (left, right) = line.split_once('~').unwrap();
        let nums: Vec<&str> = left.split(',').collect();
        b.x = nums[0].parse::<i32>().unwrap();
        b.y = nums[1].parse::<i32>().unwrap();
        b.z = nums[2].parse::<i32>().unwrap();

        let nums: Vec<&str> = right.split(',').collect();
        b.x2 = nums[0].parse::<i32>().unwrap();
        b.y2 = nums[1].parse::<i32>().unwrap();
        b.z2 = nums[2].parse::<i32>().unwrap();

        bricks.push(b);
    }

    bricks.sort_by(|b1,b2| {
        let minb1 = min(b1.z, b1.z2);
        let minb2 = min(b2.z, b2.z2);

        minb1.cmp(&minb2)
    });

    let mut children = HashMap::new();
    let mut parents = HashMap::new();
    for b in &mut bricks {
        let mut delta = (b.x2 - b.x, b.y2 - b.y, b.z2 - b.z);
        if delta.0 != 0 {
            delta.0 /= delta.0.abs();
        }
        if delta.1 != 0 {
            delta.1 /= delta.1.abs();
        }
        if delta.2 != 0 {
            delta.2 /= delta.2.abs();
        }
        
        let mut falling = true;
        while falling {
            let mut start = (b.x, b.y, b.z);
            let end = (b.x2, b.y2, b.z2);
            loop {
                let pos = (start.0, start.1, start.2-1);
                if hmap.contains_key(&pos) {
                    falling = false;
                    let val: usize = *hmap.get(&pos).unwrap();
                    children.entry(b.id).or_insert_with(HashMap::new).insert(val, true);
                    parents.entry(val).or_insert_with(HashMap::new).insert(b.id, true);
                }
                
                if start == end {
                    break;
                }
                
                start = (start.0 + delta.0, start.1 + delta.1, start.2 + delta.2);
            }

            if falling {
                if b.z == 1 || b.z2 == 1 {
                    falling = false;
                } else {
                    b.z -= 1;
                    b.z2 -= 1;
                }
            }
        }

        let mut start = (b.x, b.y, b.z);
        let end = (b.x2, b.y2, b.z2);
        loop {
            hmap.insert(start, b.id);
            if start == end {
                break;
            }
            start = (start.0 + delta.0, start.1 + delta.1, start.2 + delta.2);
        }
    }

    let mut cant_remove = HashMap::new();
    for (k,v) in &children {
        if v.len() == 1 {
            for (kk, vv) in v {
                cant_remove.insert(kk, true);
            }
        } else if v.len() == 0 {
            //println!("{} on ground", k);
        }
    }

    for b in &bricks {
        let mut moved: HashMap<usize, bool> = HashMap::new();
        calc(&parents, &children, &mut moved, b.id);
        sum2 += moved.len() as i64 - 1;
    }

    sum = (bricks.len() - cant_remove.len()) as i64;
    println!("ans: {} {}", sum, sum2);
}
