use std::collections::HashMap;

fn calc(
    mut pos: (i64, i64),
    mut visited: HashMap<(i64, i64), bool>,
    map: &Vec<Vec<char>>,
) -> i64 {

    if pos.1 == map.len() as i64 - 1 {
        return 0;
    }

    let c = map[pos.1 as usize][pos.0 as usize];
    if c == '<'  {
        let newpos = (pos.0 - 1, pos.1 + 0);
        if visited.contains_key(&newpos) {
            return -99999999999;
        }
        visited.insert(newpos, true);
        return calc((pos.0-1,pos.1), visited, map)+1;
    }
    if c == '>' {
        let newpos = (pos.0 + 1, pos.1 + 0);
        if visited.contains_key(&newpos) {
            return -99999999999;
        }
        visited.insert(newpos, true);
        return calc(newpos, visited, map)+1;
    }
    if c == 'v' {
        let newpos = (pos.0 - 0, pos.1 + 1);
        if visited.contains_key(&newpos) {
            return -99999999999;
        }
        visited.insert(newpos, true);
        return calc(newpos, visited, map)+1;
    }
    if c == '^' {
        let newpos = (pos.0 - 0, pos.1 - 1);
        if visited.contains_key(&newpos) {
            return -99999999999;
        }
        visited.insert(newpos, true);
        return calc(newpos, visited, map)+1;
    }
    let mut max = -9999999999;
    let mut once = true;
    let mut sum = 0;

    let mut queue = Vec::new();
    while once {
        queue = Vec::new();
        for (x, y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let newpos = (pos.0 + x, pos.1 + y);
            if newpos.0 >= map[0].len() as i64
                || newpos.1 >= map.len() as i64
                || newpos.0 < 0
                || newpos.1 < 0
            {
                continue;
            }
            if map[newpos.1 as usize][newpos.0 as usize] == '#' {
                continue;
            }
            if visited.contains_key(&newpos) {
                continue;
            }
            queue.push(newpos);
        }

        if queue.len() == 0 {
            return -9000999;
        }
        if queue.len() == 1 {
            sum += 1;
            pos = queue[0];
            visited.insert(pos, true);

            if pos.1 == map.len() as i64 - 1 {
                return sum;
            }
        } else {
            once = false;
            break;
        }
    }

    for newpos in queue {
        let mut newvisited = visited.clone();
        newvisited.insert(newpos, true);
        let res = calc(newpos, newvisited, map);
        if max < res {
            max = res;
        }
    }

    let retval = sum + max + 1;

    return retval;
}

fn calc3(n: usize, costs: &HashMap<usize,Vec<(usize,i64)>>, mut visited: i64, cache: &mut HashMap<(usize,i64),i64>, end: usize) -> i64 {
    if n == end {
        return 0;
    }
    
    if visited & (1 << n) > 0 {
        return -99999999999;
    }
    
    visited |= 1 << n;
    
    let key = (n, visited);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    let mut max = -99999999999;
    for (nn, dist) in costs.get(&n).unwrap() {
        let res = calc3(*nn, costs, visited, cache, end);
        if max < dist + res {
            max = dist + res;
        }
    }

    cache.insert(key,max);
    return max;
}

fn main() {
    let input = include_str!("input23.txt");
    let mut visited = HashMap::new();
    let mut map = Vec::new();
    let mut sum2 = 0;
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c != '#' {
                sum2 += 1;
            }
            row.push(c);
        }
        map.push(row);
    }

    let mut keys: HashMap<(i64,i64),bool> = HashMap::new();
    let mut cross: HashMap<(i64, i64), Vec<((i64, i64), i64)>> = HashMap::new();
    let mut end = (0,0);
    let mut enddist = 0;
    let mut startdist = 0;
    let mut start = (0,0);
    let mut nodes: Vec<(i64,i64)> = Vec::new();
    let mut costs = HashMap::new();
    let mut starti = 0;
    let mut endi = 0;
    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            if map[y][x] == '#' {
                continue;
            }
            let mut num = 0;
            if map[y + 1][x] != '#' {
                num += 1;
            }
            if map[y - 1][x] != '#' {
                num += 1;
            }
            if map[y][x + 1] != '#' {
                num += 1;
            }
            if map[y][x - 1] != '#' {
                num += 1;
            }

            if num > 2 {
                nodes.push((x as i64,y as i64));
                costs.insert(nodes.len()-1, Vec::new());
                keys.insert((x as i64, y as i64), true);
                cross.insert((x as i64, y as i64), Vec::new());
            }
        }
    }
    
    for i in 0..nodes.len() {
        let c = costs.get_mut(&i).unwrap();
        let mut queue: Vec<(i64,i64)> = Vec::new();
        queue.push(nodes[i]);
        let mut dist = 0;
        let mut visited = HashMap::new();
        visited.insert(queue[0], true);
        
        while queue.len() > 0 {
            dist += 1;
            let mut nq = Vec::new();
            for q in queue {
                for (x,y) in [(1,0),(-1,0),(0,1),(0,-1)] {
                    let newpos = (q.0+x,q.1+y);
                    if newpos.0 < 0 || newpos.1 >= map[0].len() as i64 ||
                       newpos.1 < 0 || newpos.1 >= map.len() as i64 {
                           continue;
                       }
                    if visited.contains_key(&newpos) {
                        continue;
                    }
                    if map[newpos.1 as usize][newpos.0 as usize] == '#' {
                        continue;
                    }

                    if keys.contains_key(&newpos) {
                        let index = nodes.iter().position(|&r| r == newpos).unwrap();
                        c.push((index, dist));
                        continue;
                    }
                    
                    if newpos.1 == map.len() as i64 - 1 {
                        endi = i;
                        enddist = dist;
                        continue;
                    }
                    
                    if newpos.1 == 0 {
                        starti = i;
                        startdist = dist;
                        continue;
                    }
                    
                    visited.insert(newpos, true);
                    nq.push(newpos);
                }
            }
            queue = nq;
        }
    }

    visited.insert((1, 0), true);
    let mut cache = HashMap::new();
    sum2 = calc3(starti, &costs, 0, &mut cache, endi)+startdist+enddist;
    let sum = calc((1, 0), visited, &map);
    println!("{} {}", sum, sum2);
}
