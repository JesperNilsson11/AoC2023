
fn main() {
    let input = include_str!("input2.txt");
    let mut sum = 0;

    for (i, line) in input.split('\n').enumerate() {
        let l = &line[line.find(":").unwrap()+2..line.len()];
        let mut _valid = true;

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for cube in l.split([',', ';']) {
            
            let v: Vec<&str> = cube.trim().split(' ').collect();
            let num = v[0].parse::<i32>().unwrap();
            
            println!("{} {}", num, v[1]);
            match v[1] {
                "red" => if num > red {
                    red = num;
                } 
                "green" => if num > green {
                    green = num;
                }
                "blue" => if num > blue {
                    blue = num;
                }
                _ => ()
            }
            /*match v[1] {
                "red" => if num > 12 {
                    valid = false;
                    break;
                } 
                "green" => if num > 13 {
                    valid = false;
                    break;
                }
                "blue" => if num > 14 {
                    valid = false;
                    break;
                }
                _ => {
                    valid = false;
                    break;
                }
            }*/
        }
        sum += red*green*blue;
        /*if valid {
            sum += i + 1;
        }*/
    }

    println!("{}", sum);
}