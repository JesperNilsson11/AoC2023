use std::collections::HashMap;

struct Item {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

fn func(conditions: &Vec<(String, String)>, item: &Item) -> String {
    for (c, target) in conditions {
        if c == "" {
            return target.to_string();
        }

        let num = (&c[2..]).parse::<i64>().unwrap();
        let var = &c[0..1];
        let op = &c[1..2];

        match var {
            "x" => {
                match op {
                    "<" => if item.x < num { return target.to_string(); },
                    ">" => if item.x > num { return target.to_string(); },
                    _ => println!("op error"),
                }
            },
            "m" => {
                match op {
                    "<" => if item.m < num { return target.to_string(); },
                    ">" => if item.m > num { return target.to_string(); },
                    _ => println!("op error"),
                }
            },
            "a" => {
                match op {
                    "<" => if item.a < num { return target.to_string(); },
                    ">" => if item.a > num { return target.to_string(); },
                    _ => println!("op error"),
                }
            },
            "s" => {
                match op {
                    "<" => if item.s < num { return target.to_string(); },
                    ">" => if item.s > num { return target.to_string(); },
                    _ => println!("op error"),
                }
            },
            _ => println!("var error"),
        }
    }

    return "R".to_string();
}

fn func2(hmap: &HashMap<String, Vec<(String, String)>>, target: String, mut x: (i32, i32), mut m: (i32, i32), mut a: (i32, i32), mut s: (i32, i32)) -> i64 {
    let mut sum = 0;
    if target == "A" {
        let xd = (x.1-x.0+1) as i64;
        let md = (m.1-m.0+1) as i64;
        let ad = (a.1-a.0+1) as i64;
        let sd = (s.1-s.0+1) as i64;

        return xd * md * ad * sd;
    }
    if target == "R" {
        return 0;
    }

    for (c, target) in hmap.get(&target).unwrap() {
        if c == "" {
            return sum + func2(hmap, target.to_string(), x, m, a, s);
        }

        let num = (&c[2..]).parse::<i32>().unwrap();
        let var = &c[0..1];
        let op = &c[1..2];

        match var {
            "x" => {
                match op {
                    "<" => if x.0 < num {
                        if x.1 < num {
                            return sum + func2(hmap, target.to_string(), x, m, a, s); 
                        }

                        let newx = (x.0, num-1);
                        sum += func2(hmap, target.to_string(), newx, m, a, s);
                        x.0 = num;
                    },
                    ">" => if x.1 > num { 
                        if x.0 > num {
                            return sum + func2(hmap, target.to_string(), x, m, a, s); 
                        }

                        let newx = (num+1, x.1);
                        sum += func2(hmap, target.to_string(), newx, m, a, s);
                        x.1 = num;
                    },
                    _ => println!("op error"),
                }
            },
            "m" => {
                match op {
                    "<" => if m.0 < num {
                        if m.1 < num {
                            return sum + func2(hmap, target.to_string(), x, m, a, s); 
                        }

                        let newm = (m.0, num-1);
                        sum += func2(hmap, target.to_string(), x, newm, a, s);
                        m.0 = num;
                    },
                    ">" => if m.1 > num { 
                        if m.0 > num {
                            return sum + func2(hmap, target.to_string(), x, m, a, s); 
                        }

                        let newm = (num+1, m.1);
                        sum += func2(hmap, target.to_string(), x, newm, a, s);
                        m.1 = num;
                    },
                    _ => println!("op error"),
                }
            },
            "a" => {
                match op {
                    "<" => if a.0 < num {
                        if a.1 < num {
                            return sum + func2(hmap, target.to_string(), x, m, a, s); 
                        }

                        let newa = (a.0, num-1);
                        sum += func2(hmap, target.to_string(), x, m, newa, s);
                        a.0 = num;
                    },
                    ">" => if a.1 > num { 
                        if a.0 > num {
                            return sum + func2(hmap, target.to_string(), x, m, a, s); 
                        }

                        let newa = (num+1, a.1);
                        sum += func2(hmap, target.to_string(), x, m, newa, s);
                        a.1 = num;
                    },
                    _ => println!("op error"),
                }
            },
            "s" => {
                match op {
                    "<" => if s.0 < num {
                        if s.1 < num {
                            return sum + func2(hmap, target.to_string(), x, m, a, s); 
                        }

                        let news = (s.0, num-1);
                        sum += func2(hmap, target.to_string(), x, m, a, news);
                        s.0 = num;
                    },
                    ">" => if s.1 > num { 
                        if s.0 > num {
                            return sum + func2(hmap, target.to_string(), x, m, a, s); 
                        }

                        let news = (num+1, s.1);
                        sum += func2(hmap, target.to_string(), x, m, a, news);
                        s.1 = num;
                    },
                    _ => println!("op error"),
                }
            },
            _ => println!("var error"),
        }
    }

    return sum;
}

fn main() {
    let input = include_str!("input19.txt");
    let mut sum: i64 = 0;
    let mut sum2: i64 = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut hmap = HashMap::new();
    
    let (instructions, items) = input.trim().split_once("\n\n").unwrap();

    for line in instructions.trim().lines() {
        let (name, rest) = line.split_once('{').unwrap();
        let mut conditions = Vec::new();
        for cond in rest.split(',') {
            let opt = cond.split_once(':');

            if opt == None {
                conditions.push(("".to_string(), (cond[0..cond.len()-1]).to_string()));
            } else {
                let (c, goto) = opt.unwrap();

                conditions.push((c.to_string(), goto.to_string()));
            }
        }

        hmap.insert(name.to_string(), conditions);
    }

    for line in items.lines() {
        let mut i: Item = Item{x: 0, m: 0, a: 0, s:0 };
        let sl = &line[1..line.len()-1];

        for eq in sl.split(',') {
            let (var, num) = eq.split_once('=').unwrap();

            
            let n = num.parse::<i64>().unwrap();

            match var {
                "x" => i.x = n,
                "m" => i.m = n,
                "a" => i.a = n,
                "s" => i.s = n,
                _ => println!("error"),
            }
        }

        let mut target = "in".to_string();
        while target != "A" && target != "R" {
            target = func(hmap.get(&target).unwrap(), &i);
            //println!("{}", target);
        }

        if target == "A" {
            sum += i.x;
            sum += i.m;
            sum += i.a;
            sum += i.s;
        }
    }

    
    sum2 = func2(&hmap, "in".to_string(), (1, 4000), (1, 4000), (1, 4000), (1, 4000));

    println!("ans: {} {}", sum, sum2);
}
