use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input = include_str!("input10.txt");
    let mut sum: i64 = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut hmap = HashMap::new();
    let mut s_found = false;
    let mut pos: (usize, usize) = (0, 0);
    let mut file = File::create("output.txt").unwrap();
    
    for line in input.trim().lines() {
        let mut row: Vec<char> = Vec::new();

        if s_found == false {
            pos.0 = 0;
        }

        for c in line.chars() {
            if c == 'S' {
                s_found = true;
            }
            row.push(c);
            if s_found == false {
                pos.0 += 1;
            }
        }
        map.push(row);
        if s_found == false {
            pos.1 += 1;
        }
    }

    let mut queue = Vec::new();
    if pos.0 > 0 {
        let left = map[pos.1][pos.0-1];
        if left == '-' || left == 'F' || left == 'L' {
            queue.push((pos.0-1, pos.1));
        }
    }

    if pos.1 > 0 {
        let up = map[pos.1-1][pos.0];
        if up == '|' || up == '7' || up == 'F' {
            if pos.1 > 0 {
                queue.push((pos.0, pos.1-1));
            }
        }
    }
    let right = map[pos.1][pos.0+1];
    if right == '-' || right == 'J' || right == '7' {
        queue.push((pos.0+1, pos.1));
    }
    let down = map[pos.1+1][pos.0];
    if down == '|' || down == 'L' || down == 'J' {
        queue.push((pos.0, pos.1+1));
    }

    /* Hard coded value based on input */
    map[pos.1][pos.0] = '|';
    hmap.insert(pos, 0);

    while queue.len() > 0 {
        sum += 1;
        let mut newqueue = Vec::new();

        for q in &queue {
            let (x, y) = *q;
            let r = hmap.entry(*q).or_insert(sum);
            if *r != sum {
                continue;
            }
            let c = map[y][x];

            match c {
                '|' => {
                    newqueue.push((x,y+1));
                    newqueue.push((x,y-1));
                },
                '-' => {
                    newqueue.push((x+1,y));
                    newqueue.push((x-1,y));
                },
                'L' => {
                    newqueue.push((x,y-1));
                    newqueue.push((x+1,y));
                },
                'J' => {
                    newqueue.push((x,y-1));
                    newqueue.push((x-1,y));
                },
                '7' => {
                    newqueue.push((x,y+1));
                    newqueue.push((x-1,y));
                },
                'F' => {
                    newqueue.push((x,y+1));
                    newqueue.push((x+1,y));
                },
                _ =>  {
                    println!("Should not happen?");
                }
            }
        }
        queue = newqueue;
    }

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if hmap.contains_key(&(x,y)) == false {
                map[y][x] = '.';
            }
        }
    }

    /*
    let mut newmap: Vec<Vec<char>> = vec![vec!['.'; map[0].len()*3]; map.len()*3];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let mut c = map[y][x];
            if c == 'S' {
                c = '|';
            }
            
            match c {
                '|' => {
                    newmap[y*3][x*3+1] = '|';
                    newmap[y*3+1][x*3+1] = '|';
                    newmap[y*3+2][x*3+1] = '|';
                },
                '-' => {
                    newmap[y*3+1][x*3+0] = '-';
                    newmap[y*3+1][x*3+1] = '-';
                    newmap[y*3+1][x*3+2] = '-';
                },
                'L' => {
                    newmap[y*3+0][x*3+1] = '|';
                    newmap[y*3+1][x*3+1] = 'L';
                    newmap[y*3+1][x*3+2] = '-';
                },
                'J' => {
                    newmap[y*3+0][x*3+1] = '|';
                    newmap[y*3+1][x*3+1] = 'J';
                    newmap[y*3+1][x*3+0] = '-';
                },
                '7' => {
                    newmap[y*3+1][x*3+0] = '-';
                    newmap[y*3+1][x*3+1] = '7';
                    newmap[y*3+2][x*3+1] = '|';
                },
                'F' => {
                    newmap[y*3+1][x*3+2] = '-';
                    newmap[y*3+1][x*3+1] = 'F';
                    newmap[y*3+2][x*3+1] = '|';
                },
                _ =>  {
                    
                }
            }
        }
    }
    */
    /*
    */
    let mut newmap: Vec<Vec<char>> = Vec::new();
    for y in 0..map.len() {
        let mut newrow = Vec::new();
        let mut newrow2 = Vec::new();
        for x in 0..map[y].len() {
            let c = map[y][x];
            newrow.push(c);
            let mut newc = '.';
            let mut newc2 = '.';
            
            if x < map[y].len()-1 {
                let c2 = map[y][x+1];
                
                if (c == '-' || c == 'L' || c == 'F') && (c2 == '-' || c2 == 'J' || c2 == '7') {
                    newc = '-';
                }
            }
            
            if y < map.len() - 1 {
                let c2 = map[y+1][x];
                
                if (c == '|' || c == '7' || c == 'F') && (c2 == '|' || c2 == 'L' || c2 == 'J') {
                    newc2 = '|';
                }
            }
            
            newrow.push(newc);
            newrow2.push(newc2);
            newrow2.push('.');
        }
        //println!("{:?}", newrow);
        newmap.push(newrow);
        newmap.push(newrow2);
    }

    /*for y in 0..newmap.len() {
        for num in &newmap[y] {
            let _ = file.write_all(num.to_string().as_bytes());
        }
        let _ = file.write_all("\n".to_string().as_bytes());
    }*/
    
    let width = newmap[0].len()-1;
    let height = newmap.len()-1;

    for x in 0..newmap[0].len() {    
        if newmap[0][x] == '.' {
            queue.push((x,0));
            newmap[0][x] = 'O';
        }
        if newmap[height][x] == '.' {
            queue.push((x,height));
            newmap[height][x] = 'O';
        }
    }
    for y in 0..newmap.len() {
        if newmap[y][0] == '.' {
            queue.push((0,y));
            newmap[y][0] = 'O';
        }
        if newmap[y][width] == '.' {
            queue.push((width,y));
            newmap[y][width] = 'O';
        }
    }
    
    while queue.len() > 0 {
        let (x,y) = queue[0];
        queue.remove(0);

        if x > 0 && newmap[y][x-1] == '.' {
            queue.push((x-1, y));
            newmap[y][x-1] = 'O';
        }
        if x < newmap[y].len() - 1 && newmap[y][x+1] == '.' {
            queue.push((x+1, y));
            newmap[y][x+1] = 'O';
        }
        if y > 0 && newmap[y-1][x] == '.' {
            queue.push((x, y-1));
            newmap[y-1][x] = 'O';
        }
        if y < newmap.len() - 1 && newmap[y+1][x] == '.' {
            queue.push((x, y+1));
            newmap[y+1][x] = 'O';
        }
    }

    for y in 0..newmap.len() {
        for num in &newmap[y] {
            let _ = file.write_all(num.to_string().as_bytes());
        }
        let _ = file.write_all("\n".to_string().as_bytes());
    }

    let mut sum2: i64 = 0;
    for y in (0..newmap.len()).step_by(2) {
        for x in (0..newmap[y].len()).step_by(2) {
            if newmap[y][x] == '.' {
                sum2 += 1;
            }
        }
    }
   
    println!("{} {}", sum - 1, sum2);
}
