struct Range {
    dest: u64,
    src: u64,
    len: u64,
}

fn main() {
    let input = include_str!("input5.txt");
    let mut lowest: u64 = u64::MAX;
    let mut maps: Vec<Vec<Range>> = Vec::new();

    let (seeds, rest) = input.trim().split_once('\n').unwrap();

    for map in rest.trim().split("\n\n") {
        let (_, ranges) = map.split_once('\n').unwrap();
        let mut m: Vec<Range> = Vec::new();

        for line in ranges.split('\n') {
            let mut r = Range {
                dest: 0,
                src: 0,
                len: 0,
            };
            let v: Vec<&str> = line.split(' ').collect();
            r.dest = v[0].parse::<u64>().unwrap();
            r.src = v[1].parse::<u64>().unwrap();
            r.len = v[2].parse::<u64>().unwrap();
            m.push(r);
        }
        maps.push(m);
    }

    let (_, seednums) = seeds.split_once(": ").unwrap();
    /*for s in seednums.split(' ') {
        let mut seed = s.parse::<u64>().unwrap();
        
        for map in &maps {
            for range in map {
                if range.src <= seed && seed < range.src + range.len {
                    seed = range.dest + seed - range.src;
                    break;
                }
            }
        }
        if seed < lowest {
            lowest = seed;
        }
    }*/
    
    let mut seed_ranges: Vec<u64> = Vec::new();
    for s in seednums.split(' ') {
        seed_ranges.push(s.parse::<u64>().unwrap());
    }
    for idx in 0..seed_ranges.len()/2 {
        for s in seed_ranges[idx*2]..seed_ranges[idx*2]+seed_ranges[idx*2+1] {
            let mut seed = s;
            for map in &maps {
                for range in map {
                    if range.src <= seed && seed < range.src + range.len {
                        seed = range.dest + seed - range.src;
                        break;
                    }
                }
            }
            if seed < lowest {
                lowest = seed;
            }
        }
    }

    println!("{}", lowest);
}