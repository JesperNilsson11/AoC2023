fn intersect(l1: &Vec<f64>, l2: &Vec<f64>) -> bool {
    if l2[4] == 0.0 {
        //println!("dy 0");
        return false;
    }
    let sscalar = (l1[1] - l2[1]) / l2[4];
    let st = l1[4] / l2[4];
 
    let t = l1[3] - l2[3] * st;
    let tscalar = l2[0] - l1[0] + l2[3] * sscalar;
    if t == 0.0 {
        //println!("t 0");
        //println!("{:?}", l2);
        return false;
    }
    let t = tscalar / t;
    if t <= 0.0 {
        //println!("behind");
        return false;
    }
    //println!("t: {}", t);
    let x = l1[0] + l1[3] * t;
    let y = l1[1] + l1[4] * t;
 
    //if (x as i64) < min || (x.ceil() as i64) > max || (y as i64) < min || (y.ceil() as i64) > max {
        //continue;
    //}
 
    let s = sscalar + st * t;
    if s < 0.0 {
        return false;
    }
 
 
    if (l1[2]+l1[5]*t - (l2[2]+l2[5]*s)).abs() > 13.9 {
        return false;
    }
    return true;
 }
 
 fn main() {
    let input = include_str!("input24.txt");
    let mut sum = 0;
    let mut sum2 = 0;
    let mut hails = Vec::new();
 
    for line in input.trim().lines() {
        let (left, right) = line.split_once(" @ ").unwrap();
        let coord: Vec<&str> = left.split(", ").collect();
        let speed: Vec<&str> = right.split(", ").collect();
        let mut h = Vec::new();
        for i in 0..3 {
            h.push(coord[i].trim().parse::<f64>().unwrap());
        }
        for i in 0..3 {
            h.push(speed[i].trim().parse::<f64>().unwrap());
        }
        hails.push(h);
    }
    let min = 200000000000000;
    let max = 400000000000000;
    let range = 2000;
    let mut found = false;
    for i in 0..hails.len() {
        for j in i+1..hails.len() {
            let l1 = &hails[i];
            let l2 = &hails[j];
 
            if l2[4] == 0.0 {
                continue;
            }
            let sscalar = (l1[1] - l2[1]) / l2[4];
            let st = l1[4] / l2[4];
 
            let t = l1[3] - l2[3] * st;
            let tscalar = l2[0] - l1[0] + l2[3] * sscalar;
            if t == 0.0 {
                continue;
            }
            let t = tscalar / t;
            if t <= 0.0 {
                continue;
            }
            let x = l1[0] + l1[3] * t;
            let y = l1[1] + l1[4] * t;
 
            if (x as i64) < min
                || (x.ceil() as i64) > max
                || (y as i64) < min
                || (y.ceil() as i64) > max
            {
                continue;
            }
 
            let s = sscalar + st * t;
            if s < 0.0 {
                continue;
            }
            sum += 1;
        }
    }
    for n1 in 1..=range {
        for n2 in 1..=n1 {
            for (ddx, ddy) in [(n1,n2),(-n1,n2),(n1,-n2),(-n1,-n2),(n2,n1),(-n2,n1),(n2,-n1),(-n2,-n1)] {
            let dx = ddx as f64;
            let dy = ddy as f64;
            let mut l1 = hails[0].clone();
            l1[3] -= dx;
            l1[4] -= dy;
            let mut l2 = hails[1].clone();
            l2[3] -= dx;
            l2[4] -= dy;
 
            if l2[4] == 0.0 {
                continue;
            }
            let sscalar = (l1[1] - l2[1]) / l2[4];
            let st = l1[4] / l2[4];
 
            let t = l1[3] - l2[3] * st;
            let tscalar = l2[0] - l1[0] + l2[3] * sscalar;
            if t == 0.0 {
                continue;
            }
            let t = tscalar / t;
            if t <= 0.0 {
                continue;
            }
            let x = l1[0] + l1[3] * t;
            let y = l1[1] + l1[4] * t;
 
            let s = sscalar + st * t;
            if s < 0.0 {
                continue;
            }

            let mut dz = (l1[2]+l1[5]*t-(l2[2]+l2[5]*s)) / (t-s);
            dz = dz.round();
            let mut ray = Vec::new();
            ray.push((l1[0]+l1[3]*t).round());
            ray.push((l1[1]+l1[4]*t).round());
            ray.push((l1[2]+(l1[5]-dz)*t).round());
            ray.push(dx);
            ray.push(dy);
            ray.push(dz);
 
            sum2 = (ray[0]+ray[1]+ray[2]) as i64;
            found = true;
            for l in &hails {
                if intersect(l, &ray) == false {
                    found = false;
                    break;
                }
            }
            if found {
                println!("{:?}", ray);
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
    if found == false {
        println!("Did not find solution");
    }
 
    println!("{} {}", sum, sum2);
 }