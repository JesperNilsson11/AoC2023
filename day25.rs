use std::collections::HashMap;

fn main() {
    let input = include_str!("input25.txt");
    let mut sum = 0;

    let mut igraph: Vec<Vec<i32>> = Vec::new();
    let mut iedges: Vec<i32> = Vec::new();
    let mut stoi = HashMap::new();
    let mut roads = HashMap::new();

    for line in input.trim().lines() {
        let (name, rest) = line.split_once(": ").unwrap();
        let iname: i32;
        if stoi.contains_key(&name) {
            iname = *stoi.get(&name).unwrap();
        } else {
            iname = stoi.len() as i32;
            stoi.insert(name, iname);
            igraph.push(Vec::new());
        }

        for n in rest.split(' ') {
            let inn: i32;
            if stoi.contains_key(&n) {
                inn = *stoi.get(&n).unwrap();
            } else {
                inn = stoi.len() as i32;
                stoi.insert(n, inn);
                igraph.push(Vec::new());
            }
            igraph[iname as usize].push(inn);
            igraph[inn as usize].push(iname);
            if iname < inn {
                let path = iname << 16 | inn;
                iedges.push(path);
                roads.insert(path, 0);
            } else {
                let path = inn << 16 | iname;
                iedges.push(path);
                roads.insert(path, 0);
            }
        }
    }

    for i in 0..igraph.len() as i32 {
            let mut visited = HashMap::new();
            let mut queue = Vec::new();
            let mut temp = Vec::new();
            temp.push(i);
            queue.push((i, temp));
            visited.insert(i, true);

            while queue.len() > 0 {
                let mut nq = Vec::new();
                while queue.len() > 0 {
                let (key, path) = queue.pop().unwrap();

                for k in 0..path.len()-1 {
                    let p;
                    if path[k] < path[k+1] {
                        p = path[k] << 16 | path[k+1];
                    } else {
                        p = path[k+1] << 16 | path[k];
                    }
                    *roads.get_mut(&p).unwrap() += 1;
                }
                let neigbours = igraph[key as usize].clone();
                for node in neigbours {
                    if visited.contains_key(&node) {
                        continue;
                    }
                    let mut pp = path.clone();
                    pp.push(node);
                    visited.insert(node,true);
                    nq.push((node,pp));
                }
            }
                queue = nq;
            }
    }

    let mut vals = Vec::new();
    for (k,v) in roads {
        vals.push((v,k));
    }
    vals.sort();
    let w = vals.len();
    let e1 = vals[w-1].1;
    let e2 = vals[w-2].1;
    let e3 = vals[w-3].1;

    let mut found = false;
    for i in 0..iedges.len() {
        for j in i+1..iedges.len() {
            for _k in j+1..iedges.len() {
                let mut groups = 0;
                let mut visited = HashMap::new();
                let mut nums = Vec::new();

                while groups < 2 {
                    let mut queue = Vec::new();
                    for key in 0..igraph.len() as i32 {
                        if visited.contains_key(&key) {
                            continue;
                        }
                        queue.push(key);
                        visited.insert(key, true);
                        break;
                    }

                    while queue.len() > 0 {
                        let node = queue.pop().unwrap();
                        let vec = &igraph[node as usize];
                        for n2 in vec {
                            if visited.contains_key(&n2) {
                                continue;
                            }
                            let ie;
                            if node < *n2 {
                                ie = node << 16 | *n2;
                            } else {
                                ie = *n2 << 16 | node;
                            }
                            if ie == e1 || ie == e2 || ie == e3 {
                                continue;
                            }

                            visited.insert(*n2, true);
                            queue.push(*n2);
                        }
                    }
                    groups += 1;
                    nums.push(visited.len());
                    if visited.len() == igraph.len() {
                        break;
                    }
                }
                if visited.len() == igraph.len() && nums.len() == 2 {
                    found = true;
                    sum = nums[0] * (nums[1]-nums[0]);
                    break;
                }
            }
            if found {
                break;
            }
        }
        if found {
            break;
        }
    }

    println!("ans: {}", sum);
}