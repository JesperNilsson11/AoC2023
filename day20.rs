use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Module {
    kind: i32,
    state: i32,
    inputs: HashMap<String, i32>,
}

fn main() {
    let input = include_str!("input20.txt");
    let mut sum: i64 = 0;
    let mut sum2: i64 = 0;
    //let mut map: Vec<Vec<char>> = Vec::new();
    let mut hmap = HashMap::new();
    let mut modules = HashMap::new();
    let mut node = String::new();

    for line in input.trim().lines() {
        let (left, right) = line.split_once('>').unwrap();

        let mut special = 0;
        let mut name = String::new();
        for c in left.chars() {
            if c == '%' {
                special = 1;
                continue;
            } else if c == '&' {
                special = 2;
                continue;
            } else if c == ' ' {
                break;
            }

            name.push(c);
        }

        let mut dest = Vec::new();
        for d in right.trim().split(", ") {
            if d == "rx" {
                node = name.clone();
            }
            dest.push(d);
        }

        hmap.insert(name.to_string(), dest);
        modules.insert(name.to_string(), Module{kind: special, state: 0, inputs: HashMap::new()});
    }

    hmap.insert("output".to_string(), Vec::new());
    modules.insert("output".to_string(), Module{kind: 3, state: 0, inputs: HashMap::new()});
    hmap.insert("rx".to_string(), Vec::new());
    modules.insert("rx".to_string(), Module{kind: 3, state: 0, inputs: HashMap::new()});

    for (k,v) in &hmap {
        for dest in v {
            let mut m = modules.get_mut(dest.clone()).unwrap();
            m.inputs.insert(k.clone(), 0);
        }
    }

    let mut cycles = HashMap::new();

    for i in 1..100000 {
        let mut queue = Vec::new();
        queue.push(("broadcaster", 0, "button"));
        let mut found = false;

        while queue.len() > 0 {
            let (name, signal, from) = queue[0];
            //println!("{} {} {}", from, signal, name);
            queue.remove(0);
            if signal == 0 && i <= 1000 {
                sum += 1;
            } else if i <= 1000 {
                sum2 += 1;
            }

            if name == node && signal == 1 {
                if cycles.contains_key(&from) {
                    found = true;
                    break;
                }

                cycles.insert(from.clone(), i);
            }

            let module = modules.get_mut(name).unwrap();
            if module.kind == 0 {
                for dest in hmap.get(name).unwrap() {
                    queue.push((dest, signal, name));
                }
            } else if module.kind == 1 {
                if signal == 1 {
                    continue;
                }

                module.state = (module.state + 1) % 2;
                for dest in hmap.get(name).unwrap() {
                    queue.push((dest, module.state, name));
                }
            } else if module.kind == 2 {
                let mut input = module.inputs.get_mut(from).unwrap();
                *input = signal;

                let mut newsig = 0;
                for (_, inp) in &module.inputs {
                    if *inp == 0 {
                        newsig = 1;
                        break;
                    }
                }

                for dest in hmap.get(name).unwrap() {
                    queue.push((dest, newsig, name));
                }
            } else if module.kind == 3 && signal == 0 {
                println!("part2: {}", i);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    let mut part2 = 1 as usize;
    for (k,v) in cycles {
        part2 = lcm(part2, v as usize);
    }
    println!("ans: {} {}", sum * sum2, part2);
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
