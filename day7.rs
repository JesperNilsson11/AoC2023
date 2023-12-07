struct Bet {
    cards: Vec<i32>,
    bet: i32,
    order: i32,
}

fn main() {
    let input = include_str!("input7.txt");
    let mut sum: i32 = 0;
    let mut bets: Vec<Bet> = Vec::new();

    for line in input.trim().split('\n') {
        let (cards, bet) = line.split_once(' ').unwrap();
        //println!("{} {}", cards, bet);
        let mut v: Vec<i32> = Vec::new();
        let mut arr: [i32; 15] = [0; 15];
        for c in cards.chars() {
            if c == 'A' {
                v.push(14);
                arr[14] += 1;
            } else if c == 'K' {
                v.push(13);
                arr[13] += 1;
            } else if c == 'Q' {
                v.push(12);
                arr[12] += 1;
            } else if c == 'J' {
                v.push(1);
                arr[1] += 1;
            } else if c == 'T' {
                v.push(10);
                arr[10] += 1;
            } else {
                let num = c.to_digit(10).unwrap() as i32;
                v.push(num);
                arr[num as usize] += 1;
            }
        }

        let mut vec: Vec<i32> = Vec::new();
        let mut five = 0;
        let mut four = 0;
        let mut three = 0;
        let mut pair = 0;
        for oo in 0..5 {
            let o = 5-oo;
            for idxx in 1..14 {
                let idx = 15 - idxx;
                if arr[idx] == o {
                    match o {
                        5 =>  {
                            five = 1;
                        },
                        4 => {four = 1;},
                        3 => {three = 1;},
                        2 => {pair += 1;},
                        _ => (),
                    };
                    for _ in 0..arr[idx] {
                        vec.push(idx as i32);
                    }
                }
            }
        }
        
        /*
        let mut last = 0;
        let mut nr = 0;
        v.sort();
        for c in &v {
            if last != *c {
                if nr == 1 {
                    pair += 1;
                } else if nr == 2 {
                    three += 1;
                } else if nr == 3 {
                    four += 1;
                } else if nr == 4 {
                    five += 1;
                }
                nr = 0;
            } else {
                nr += 1;
            }
            
            last = *c;
        }
        if nr == 1 {
            pair += 1;
        } else if nr == 2 {
            three += 1;
        } else if nr == 3 {
            four += 1;
        } else if nr == 4 {
            five += 1;
        }
        */
        //println!("{:?}", vec);
        let mut order = 0;
        if five > 0 {
            order = 6;
        } else if four > 0 {
            if arr[1] > 0 {
                order = 6;
            } else {
                order = 5;
            }
        } else if three > 0 && pair > 0 {
            if arr[1] == 1 {
                order = 5;
            } else if arr[1] == 2 {
                order = 6;
            } else {
                order = 4;
            }
        } else if three > 0 {
            if arr[1] == 1 {
                order = 5;
            } else if arr[1] == 2 {
                order = 6;
            } else {
                order = 3;
            }
        } else if pair > 1 {
            if arr[1] == 1 {
                order = 4;
            } else if arr[1] == 2 {
                order = 5;
            } else if arr[1] == 3 {
                order = 6;
            } else {
                order = 2;
            }
        } else if pair > 0 {
            if arr[1] == 1 {
                order = 3;
            } else if arr[1] == 2 {
                order = 5;
            } else if arr[1] == 3 {
                order = 6;
            } else {
                order = 1;
            }
        } else {
            if arr[1] == 1 {
                order = 1;
            } else if arr[1] == 2 {
                order = 3;
            } else if arr[1] == 3 {
                order = 5;
            } else if arr[1] >= 4 {
                order = 6;
            } else {
                order = 0;
            }
        }
        bets.push(Bet{cards: v, bet: bet.parse::<i32>().unwrap(), order});
    }

    bets.sort_by(|a,b| {
        if a.order != b.order {
            return b.order.cmp(&a.order)
        }

        for idx in 0..5 {
            if a.cards[idx] != b.cards[idx] {
                return b.cards[idx].cmp(&a.cards[idx]);
            }
        }

        return b.cards[4].cmp(&a.cards[4]);
    });

    let s = bets.len();
    for (i,b) in bets.iter().enumerate() {
        println!("{} {} {:?}", (bets.len() as i32 - i as i32), b.bet, b.cards);
        sum += (bets.len() as i32 - i as i32) * b.bet;
    }

    println!("{} {}", sum, s);
}