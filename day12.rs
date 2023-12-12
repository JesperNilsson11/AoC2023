use std::collections::HashMap;

fn is_valid(map: Vec<char>, nums: &Vec<i64>) -> bool {
    let mut n = 0;
    //let mut vec = Vec::new();
    let mut idx = 0;
    for c in map {
        if c == '#' {
            n += 1;
        } else {
            if n > 0 {
                if idx >= nums.len() || n != nums[idx] {
                    return false;
                }
                //vec.push(n);
                idx += 1;
            }
            n = 0;
        }
    }
    if n > 0 {
        //vec.push(n);
        if idx >= nums.len() || n != nums[idx] {
            return false;
        }
        idx += 1;
    }
    if idx != nums.len() {
        return false;
    }

    //println!("{:?}", vec);
    /*if nums.len() != vec.len() {
        return false;
    }
    for i in 0..vec.len() {
        if vec[i] != nums[i] {
            return false;
        }
    }*/
    return true;
}

fn rec(map: Vec<char>, idx: usize, springs: i64, nums: &Vec<i64>) -> i64 {
    if idx == map.len() || springs == 0 {
        //println!("{:?}", map);
        //println!("{:?}", nums);
        if is_valid(map, nums) {
            return 1;
        }
        return 0;
    }

    let mut sum: i64 = 0;
    if map[idx] == '?' {
        let mut newmap = map.clone();
        newmap[idx] = '#';
        sum += rec(newmap.clone(), idx+1, springs - 1, &nums);
        newmap[idx] = '.';
        sum += rec(newmap, idx+1, springs, &nums);
    } else {
        sum += rec(map, idx+1, springs, &nums);
    }

    return sum;
}

fn rec2(map: Vec<char>, mut idx: usize, numidx: usize, nums: &Vec<i64>, depth: i64) -> i64 {
    if idx >= map.len() || numidx >= nums.len() {
        //println!("{:?}", map);
        //println!("{:?}", nums);
        println!("{}", depth);
        if is_valid(map, nums) {
            return 1;
        }
        return 0;
    }

    let mut sum: i64 = 0;

    while idx < map.len() && map[idx] == '.' {
        idx += 1;
    }
    if idx == map.len() {
        if is_valid(map, nums) {
            return 1;
        }
        return 0;
    }

    if map[idx] == '?' {
        let mut newmap = map.clone();
        newmap[idx] = '.';
        sum += rec2(newmap.clone(), idx, numidx, &nums, depth+1);

        let nr = nums[numidx] as usize;
        if idx+nr > newmap.len() {
            return sum;
        }
        newmap[idx] = '#';
        for i in idx..idx+nr {
            if newmap[i] == '.' {
                return sum;
            }
            newmap[i] = '#';
        }
        idx += nr;
        if idx < newmap.len() {
            if newmap[idx] == '#' {
                return sum;
            }
            newmap[idx] = '.';
        }
        sum += rec2(newmap, idx, numidx+1, &nums, depth+1);
    } else {
        let mut newmap = map.clone();
        let nr = nums[numidx] as usize;

        if idx+nr > newmap.len() {
            return 0;
        }
        for i in idx..idx+nr {
            if newmap[i] == '.' {
                return 0;
            }
            newmap[i] = '#';
        }
        idx += nr;
        if idx < newmap.len() {
            if newmap[idx] == '#' {
                return 0;
            }
            newmap[idx] = '.';
        }
        sum = rec2(newmap, idx, numidx+1, &nums, depth+1);
    }

    return sum;
}

fn main() {
    let input = include_str!("input12.txt");
    let mut sum: i64 = 0;
    //let mut map: Vec<Vec<char>> = Vec::new();
    
    for line in input.trim().lines() {
        let (m, nums) = line.split_once(' ').unwrap();

        let mut springs: Vec<char> = Vec::new();
        for c in m.chars() {
            springs.push(c);
        }
        let mut n: Vec<i64> = Vec::new();
        for strn in nums.split(',') {
            n.push(strn.parse::<i64>().unwrap());
        }

        let mut newsprings: Vec<char> = Vec::new();
        let mut newn: Vec<i64> = Vec::new();
        let mut nr: i64 = 0;
        for i in 0..5 {
            for c in &springs {
                newsprings.push(*c);
            }
            if i < 4 {
                newsprings.push('?');
            }
            for num in &n {
                nr += num;
                newn.push(*num);
            }
        }
        //println!("{:?}", springs);
        //println!("{:?}", n);
        //let s = rec(newsprings, 0, nr, &newn); 
        let s = rec2(newsprings, 0, 0, &newn, 0); 
        println!("{}", s);
        sum += s;
        //break;
    }
   
    println!("{}", sum);
}
