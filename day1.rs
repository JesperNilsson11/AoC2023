
fn main() {
    let input = include_str!("input1.txt");
    let mut sum: i32 = 0;

    for s in input.split('\n') {
        let mut first = '\0';
        let mut last = '\0';

        //println!("{}", s);
        for (i, c) in s.chars().enumerate() {
            if c >= '0' && c <= '9' {
                last = c;
                if first == '\0' {
                    first = c;
                }
            } else if s.len() >= i+3 && &s[i..i+3] == "one" {
                last = '1';
                if first == '\0' {
                    first = '1';
                }
            } else if s.len() >= i+3 && &s[i..i+3] == "two" {
                last = '2';
                if first == '\0' {
                    first = '2';
                }
            } else if s.len() >= i+5 && &s[i..i+5] == "three" {
                last = '3';
                if first == '\0' {
                    first = '3';
                }
            } else if s.len() >= i+4 && &s[i..i+4] == "four" {
                last = '4';
                if first == '\0' {
                    first = '4';
                }
            } else if s.len() >= i+4 && &s[i..i+4] == "five" {
                last = '5';
                if first == '\0' {
                    first = '5';
                }
            } else if s.len() >= i+3 && &s[i..i+3] == "six" {
                last = '6';
                if first == '\0' {
                    first = '6';
                }
            } else if s.len() >= i+5 && &s[i..i+5] == "seven" {
                last = '7';
                if first == '\0' {
                    first = '7';
                }
            } else if s.len() >= i+5 && &s[i..i+5] == "eight" {
                last = '8';
                if first == '\0' {
                    first = '8';
                }
            } else if s.len() >= i+4 && &s[i..i+4] == "nine" {
                last = '9';
                if first == '\0' {
                    first = '9';
                }
            }
        }

        //println!("f:{} l:{}", first, last);

        let d1 = first as u32 - '0' as u32;
        let d2 = last as u32 - '0' as u32;
        let t = d1 * 10 + d2;
        println!("{}", t);
        sum += t as i32;
    }

    println!("{}", sum);
}