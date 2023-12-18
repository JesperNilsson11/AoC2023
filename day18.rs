use std::collections::HashMap;

fn main() {
    let input = include_str!("input18.txt");
    let mut sum: i64 = 0;
    let mut sum2: i64 = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut hmap = HashMap::new();
    let mut visited = HashMap::new();
    let mut hmap2 = HashMap::with_capacity(10000);;
    let mut visited2 = HashMap::with_capacity(10000);;
    
    let mut x = 0;
    let mut y = 0;
    let mut x2 = 0;
    let mut y2 = 0;
    for line in input.trim().lines() {
        let (dir, rest) = line.split_once(' ').unwrap();
        let (num, rest) = rest.split_once(' ').unwrap();
        let n = num.trim().parse::<i64>().unwrap();
        let (_,hex) = line.split_once('#').unwrap();
        let dir2 = &hex[5..6];
        let n2 = i64::from_str_radix(&hex[0..5], 16).unwrap();

        for _ in 0..n {
            match dir {
                "U" => y -= 1,
                "R" => x += 1,
                "D" => y += 1,
                "L" => x -= 1,
                _ => println!("dir error"),
            }

            //hmap.insert((x,y), '#');
            hmap.entry(y).or_insert_with(Vec::new).push(x);
            visited.insert((x,y), 1);
            sum += 1;
        }
        for _ in 0..n2 {
            match dir2 {
                "3" => y2 -= 1,
                "0" => x2 += 1,
                "1" => y2 += 1,
                "2" => x2 -= 1,
                _ => println!("dir error"),
            }

            hmap2.entry(y2).or_insert_with(Vec::new).push(x2);
            visited2.insert((x2,y2), 1);
            sum2 += 1;
        }
    }

    for (k, mut v) in hmap {
        let mut ranges = Vec::new();
        v.sort();
        let mut prev = v[0];
        let mut r = (prev, prev);
        for i in 1..v.len() {
            if v[i] == r.1+1 {
                r.1 = v[i];
                continue;
            }

            ranges.push(r);
            r = (v[i], v[i]);
        }
        ranges.push(r);

        let mut outside = true;
        for i in (0..ranges.len()-1) {
            if ranges[i].0 == ranges[i].1 {
                outside = !outside;
            } else {
                if visited.contains_key(&(ranges[i].1, k-1)) {
                    outside = !outside;
                }
                if visited.contains_key(&(ranges[i].0, k-1)) {
                    outside = !outside;
                }
            }

            if outside == false {
                sum += (ranges[i+1].0 - ranges[i].1 - 1);
            }
        }
    }

    for (k, mut v) in hmap2 {
        let mut ranges = Vec::new();
        v.sort();
        let mut prev = v[0];
        let mut r = (prev, prev);
        for i in 1..v.len() {
            if v[i] == r.1+1 {
                r.1 = v[i];
                continue;
            }

            ranges.push(r);
            r = (v[i], v[i]);
        }
        ranges.push(r);

        let mut outside = true;
        for i in (0..ranges.len()-1) {
            if ranges[i].0 == ranges[i].1 {
                outside = !outside;
            } else {
                if visited2.contains_key(&(ranges[i].1, k-1)) {
                    outside = !outside;
                }
                if visited2.contains_key(&(ranges[i].0, k-1)) {
                    outside = !outside;
                }
            }

            if outside == false {
                sum2 += (ranges[i+1].0 - ranges[i].1 - 1);
            }
        }
    }

    println!("ans: {} {}", sum, sum2);
}
