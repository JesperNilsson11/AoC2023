use std::collections::HashMap;

fn main() {
    let input = include_str!("input11.txt");
    let mut sum: i64 = 0;
    let mut sum2: i64 = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut hmap = HashMap::new();
    let mut galaxy = Vec::new();
    let mut expand_row: Vec<i64> = Vec::new();
    let mut expand_column: Vec<i64> = Vec::new();
    
    let mut ex = 0;
    for (y, line) in input.trim().lines().enumerate() {
        let mut row = Vec::new();
        let mut should_expand = true;
        for (x, c) in line.chars().enumerate() {
            row.push(c);
            if c == '#' {
                galaxy.push((x,y));
                hmap.insert((x,y), 0);
                should_expand = false;
            }
        }
        if should_expand {
            ex += 1000000-1;
        }
        expand_row.push(ex * 1);
        map.push(row);
    }

    ex = 0;
    expand_column = vec![0; map[0].len()];
    for x in 0..map[0].len() {
        let mut should_expand = true;
        for y in 0..map.len() {
            if map[y][x] == '#' {
                should_expand = false;
                break;
            }
        }

        if should_expand {
            ex += 1000000-1;
        }
        expand_column[x] = ex * 1;
    }

    //println!("{:?}", galaxy);
    //println!("{:?}", expand_column);
    //println!("{:?}", expand_row);
    for i in 0..galaxy.len() {
        for j in i+1..galaxy.len() {
            let manx = ((galaxy[i].0 as i64 + expand_column[galaxy[i].0] as i64 - (galaxy[j].0 as i64 + expand_column[galaxy[j].0])) as i64).abs();
            let many = ((galaxy[i].1 as i64 + expand_row[galaxy[i].1] as i64 - (galaxy[j].1 as i64 + expand_row[galaxy[j].1])) as i64).abs();
            //println!("{} {} dist: {} {} = {}", i, j, manx, many, manx+many);
            sum += manx + many;
        }
    }
   
    println!("{}", sum);
}
