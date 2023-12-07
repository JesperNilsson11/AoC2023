
fn main() {
    let mut sum: i32 = 1;
    let time: Vec<i32> = vec![49, 78, 79, 80];
    let dist: Vec<i32> = vec![298, 1185, 1066, 1181];

    /*for i in 0..time.len() {
        let mut win = 0;

        let t = time[i];
        for prep in 1..t {
            if prep * (t-prep) > dist[i] {
                win += 1;
            }
        }

        sum *= win;
    }*/

    let mut win = 0 as u64;
    let t = 49787980 as u64;
    for prep in 1..t {
        if prep * (t-prep) > 298118510661181 as u64 {
            win += 1;
        }
    }

    println!("{}", win);
}