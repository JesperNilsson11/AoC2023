use std::collections::HashMap;

fn main() {
    let input = include_str!("input15.txt");
    let mut sum: i64 = 0;
    //let mut map: Vec<Vec<char>> = Vec::new();
    //let mut hmap = HashMap::new();
    let mut boxes: Vec<Vec<(String, i64)>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    
    for line in input.trim().lines() {
        for val in line.split(',') {
            let mut hash: i64 = 0;
            let name = &val[0..val.len()];
            let mut minus = 0;
            for b in name.bytes() {
                if b == b'=' {
                    minus = 2;
                    break;
                }
                if b == b'-' {
                    minus = 1;
                    break;
                }
                hash += b as i64;
                hash *= 17;
                hash %= 256;
            }
            
            //println!("name: {} sign: {}", name, minus);
            if minus == 2 {
                let num = name[name.len()-1..].parse::<i64>().unwrap();
                //println!("{}", num);
                let mut found = false;
                let str_name = name[0..name.len()-2].to_string(); 
                for p in boxes[hash as usize].iter_mut() {
                    if p.0 == str_name {
                        found = true;
                        p.1 = num;
                        //println!("replace {}", str_name);
                        break;
                    }
                }
                if found == false {
                    //println!("push {}", str_name);
                    boxes[hash as usize].push((str_name, num));
                }
            } else if minus == 1 {
                //println!("before {:?}", boxes[hash as usize]);
                boxes[hash as usize].retain(|p| p.0 != name[0..name.len()-1]);
                //println!("after {:?}", boxes[hash as usize]);
            } else {
                println!("Error");
            }
            //sum += hash;
        }
    }

    for idx in 0..boxes.len() {
        for slot in 0..boxes[idx].len() {
            sum += (idx as i64 +1) * (slot as i64 +1) * boxes[idx][slot].1;
        }
    }
    
    println!("ans: {}", sum);
}
