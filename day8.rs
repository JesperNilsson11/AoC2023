use std::collections::HashMap;

fn main() {
    let input = include_str!("input8.txt");
    let mut sum: i32 = 0;
    let mut map = HashMap::new();
    let mut nodes: Vec<&str> = Vec::new();
    
    let (dir, rest) = input.trim().split_once("\n\n").unwrap();

    for line in rest.lines() {
        let (key, values) = line.split_once(" = ").unwrap();
        let (l, r) = values.split_once(", ").unwrap();
        map.insert(key, (&l[1..4], &r[0..3]));
        
        if key.chars().nth(2).unwrap() == 'A' {
            //println!("start node: {}", key);
            nodes.push(key);
        }
    }

    let mut idx = 0;
    /*let mut node = "AAA";
    while node != "ZZZ" {
        let d = dir.chars().nth(idx).unwrap();
        sum += 1;
        idx = (idx+1) % dir.len();

        if d == 'L' {
            node = map.get(node).unwrap().0;
        } else {
            node = map.get(node).unwrap().1;
        }

        //println!("{}", node);
    }*/
    let mut loop_cnt: Vec<i32> = Vec::new();
    for _ in &nodes {
        loop_cnt.push(0);
    }
    loop {
        let d = dir.chars().nth(idx).unwrap();
        sum += 1;
        idx = (idx+1) % dir.len();

        for j in 0..nodes.len() {
            if d == 'L' {
                nodes[j] = map.get(nodes[j]).unwrap().0;
            } else {
                nodes[j] = map.get(nodes[j]).unwrap().1;
            }
            
            if nodes[j].chars().nth(2).unwrap() != 'Z' {
                //end = false;
            } else if loop_cnt[j] == 0 {
                loop_cnt[j] = sum;
            }
        }
        
        let mut end = true;
        for c in &loop_cnt {
            if *c == 0 {
                end = false;
                break;
            }
        }
        if end == true {
            break;
        }
    }

    let mut cnt: usize = 1;
    for c in loop_cnt {
        cnt = lcm(cnt, c as usize);
    }

    //println!("dir: {}", dir);
    //println!("rest {}", rest);

    println!("{}", cnt);
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}