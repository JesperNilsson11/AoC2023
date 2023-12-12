use std::collections::HashMap;

fn is_valid(map: Vec<char>, nums: &Vec<i64>) -> bool {
    let mut n = 0;
    let mut vec = Vec::new();
    for c in map {
        if c == '#' {
            n += 1;
        } else {
            if n > 0 {
                vec.push(n);
            }
            n = 0;
        }
    }
    if n > 0 {
        vec.push(n);
    }

    //println!("{:?}", vec);
    if nums.len() != vec.len() {
        return false;
    }
    for i in 0..vec.len() {
        if vec[i] != nums[i] {
            return false;
        }
    }
    return true;
}

fn rec(map: Vec<char>, idx: usize, nums: &Vec<i64>) -> i64 {
    if idx == map.len() {
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
        sum += rec(newmap.clone(), idx+1, &nums);
        newmap[idx] = '.';
        sum += rec(newmap, idx+1, &nums);
    } else {
        sum += rec(map, idx+1, &nums);
    }

    return sum;
}

fn main() {
    let input = include_str!("input.txt");
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
        for i in 0..5 {
            for c in &springs {
                newsprings.push(*c);
            }
            if i < 4 {
                newsprings.push('?');
            }
            for num in &n {
                newn.push(*num);
            }
        }
        //println!("{:?}", springs);
        //println!("{:?}", n);
        let s = rec(springs, 0, &n); 
        println!("{}", s);
        sum += s;
        //break;
    }
   
    println!("{}", sum);
}
