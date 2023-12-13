use std::collections::HashMap;

fn main() {
    let input = include_str!("input13.txt");
    let mut sum: i64 = 0;
    
    for smap in input.trim().split("\n\n") {
        let mut map: Vec<Vec<char>> = Vec::new();
        
        for srow in smap.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in srow.chars() {
                row.push(c);
            }
            map.push(row);
        }

        for y in 0..map.len()-1 {
            let mut r = y;
            let mut reflect = 0;
            let mut ydiff = y;
            if ydiff > map.len()-2-y {
                ydiff = map.len()-2-y;
            }

            for diff in 0..=ydiff  {
                for x in 0..map[y].len() {
                    if map[y-diff][x] != map[y+1+diff][x] {
                        //reflect = false;
                        //break;
                        reflect += 1;
                        r = y;
                    }
                }

                //if reflect == false {
                //    break;
                //}
            }

            if reflect == 1 {
                //sum += 100 * (y as i64 + 1);
                //println!("{}", y+1);
                sum += 100 * (r as i64 + 1);
                //println!("{}", r+1);
            }
        }

        for x in 0..map[0].len()-1 {
            let mut reflect = 0;
            let mut xdiff = x;
            let mut c = x;

            if xdiff > map[0].len()-2-x {
                xdiff = map[0].len()-2-x;
            }

            for diff in 0..=xdiff {
                for y in 0..map.len() {
                    if map[y][x-diff] != map[y][x+1+diff] {
                        //reflect = false;
                        //break;
                        reflect += 1;
                        c = x;
                    }
                }
                //if reflect == false {
                //    break;
                //}
            }

            if reflect == 1 {
                //sum += x as i64 + 1;
                //println!("{}", x+1);
                sum += c as i64 + 1;
                //println!("{}", c+1);
            }
        }
    }
   
    println!("sum: {}", sum);
}
