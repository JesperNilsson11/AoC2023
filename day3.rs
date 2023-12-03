use std::collections::HashMap;

fn main() {
    let input = include_str!("input3.txt");
    let mut sum = 0;
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut gears = HashMap::new();
    
    for l in input.split('\n') {
        let mut temp = Vec::new();
        for c in l.bytes() {
            temp.push(c);
        }
        map.push(temp);
    }

    let mut index = 0;
    let width = map[0].len();
    let height = map.len();
    while index < height * width {
        let w = index % width;
        let h = index / height;

        if map[h][w].is_ascii_digit() {
            let mut num = map[h][w] as i32 - '0' as i32;

            let mut w1 = w;
            while w1 + 1 < width && map[h][w1+1].is_ascii_digit() {
                index += 1;
                w1 += 1;
                num *= 10;
                num += map[h][w1] as i32 - '0' as i32;
            }

            let mut valid = false;

            for ww in w as i32 -1..w1 as i32+2 {
                if h == 0 {
                    break;
                }
                if ww < 0 || ww as usize >= width {
                    continue;
                }

                if map[h-1][ww as usize] != '.' as u8 {
                    valid = true;
                    //break;
                }
                if map[h-1][ww as usize] == '*' as u8 {
                    gears.entry((h as i32-1)*width as i32 +ww).or_insert_with(Vec::new).push(num);
                }
            }
            for ww in w as i32-1..w1 as i32+2 {
                if h == height - 1 {
                    break;
                }
                if ww < 0 || ww as usize >= width {
                    continue;
                }

                if map[h+1][ww as usize] != '.' as u8 {
                    valid = true;
                    //break;
                }
                if map[h+1][ww as usize] == '*' as u8 {
                    gears.entry((h as i32+1)*width as i32 +ww).or_insert_with(Vec::new).push(num);
                }
            }

            if w > 0 && map[h][w-1] != '.' as u8 {
                valid = true;

                if map[h][w-1] == '*' as u8 {
                    gears.entry(h as i32*width as i32 + w as i32 -1).or_insert_with(Vec::new).push(num);
                }
            }
            if w1 < width-1 && map[h][w1+1] != '.' as u8 {
                valid = true;

                if map[h][w1+1] == '*' as u8 {
                    gears.entry(h as i32*width as i32 + w1 as i32 +1).or_insert_with(Vec::new).push(num);
                }
            }

            if valid {
                sum += num;
            }
        }

        index += 1;
    }

    let mut sum2: u64 = 0;
    for (_key, value) in gears {
        if value.len() == 2 {
            sum2 += value[0] as u64 * value[1] as u64;
        }
    }

    println!("{} {}", sum, sum2);
}