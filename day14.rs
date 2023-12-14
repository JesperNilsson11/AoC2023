use std::collections::HashMap;

fn calc(map: &Vec<Vec<char>>) -> i64 {
    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != 'O' {
                continue;
            }
            
            sum += (map.len() - y) as i64;
        }
    }

    return sum;
}

fn main() {
    let input = include_str!("input14.txt");
    let mut sum: i64 = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut hmap = HashMap::new();
    
    for row in input.trim().lines() {
        let mut r = Vec::new();
        for c in row.chars() {
            r.push(c);
        }
        map.push(r);
    }

    for i in 0..1000000000 {
        let mut moves = 0;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] != 'O' {
                    continue;
                }
                let mut dy = y;
                while dy > 0 && map[dy-1][x] == '.' {
                    map[dy][x] = '.';
                    dy -= 1;
                    moves += 1;
                }
            map[dy][x] = 'O';
            sum += (map.len() - dy) as i64;
            }
        }
    
        for x in 0..map[0].len() {
            for y in 0..map.len() {
                if map[y][x] != 'O' {
                    continue;
                }
                let mut dx = x;
                while dx > 0 && map[y][dx-1] == '.' {
                    map[y][dx] = '.';
                    dx -= 1;
                    moves += 1;
                }
                map[y][dx] = 'O';
            }
        }
    
        for y in (0..map.len()).rev() {
            for x in 0..map[y].len() {
                if map[y][x] != 'O' {
                    continue;
                }
                let mut dy = y;
                while dy < map.len()-1 && map[dy+1][x] == '.' {
                    map[dy][x] = '.';
                    dy += 1;
                    moves += 1;
                }
                map[dy][x] = 'O';
            }
        }
    
        for x in (0..map[0].len()).rev() {
            for y in 0..map.len() {
                if map[y][x] != 'O' {
                    continue;
                }
                let mut dx = x;
                while dx < map[y].len()-1 && map[y][dx+1] == '.' {
                    map[y][dx] = '.';
                    dx += 1;
                    moves += 1;
                }
                map[y][dx] = 'O';
            }
        }

        if hmap.contains_key(&map) == true {
            let (s,idx) = *hmap.get(&map).unwrap();
            if (1000000000-1-idx) % (i-idx) == 0 {
                println!("{} {:?} {}", i, s, idx);
                sum = s;
                break;
            }
        } else {
            hmap.insert(map.clone(), (calc(&map),i));
        }
    }
    
    println!("ans: {}", sum);
}
