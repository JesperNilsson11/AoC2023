
fn main() {
    let input = include_str!("input4.txt");
    let mut sum: i32 = 0;
    let mut sum2: i32 = 0;
    let mut cards: Vec<i32> = vec![0; 200];


    for (i, line) in input.trim().split('\n').enumerate() {
        let l = &line[line.find(':').unwrap()+1..];
        let mut left = true;
        let mut lv = Vec::new();
        let mut rv = Vec::new();

        cards[i] += 1;
        //println!("{}", l);

        for num in l.trim().split(' ') {
            if num == "" {
                continue;
            }

            if num == "|" {
                left = false;
            } else if left {
                lv.push(num.parse::<i32>().unwrap());
            } else {
                rv.push(num.parse::<i32>().unwrap());
            }
        }

        let mut point = 0;
        for n in lv {
            for nn in &rv {
                if n == *nn {
                    point += 1;
                    break;
                }
            }
        }

        println!("{}", point);
        if point > 0 {
            sum += 1 << (point-1);
        }

        for idx in i+1..i+1+point {
            cards[idx] += cards[i];
        }
    }

    for n in cards {
        sum2 += n;
    }

    println!("{} {}", sum, sum2);
}