
fn main() {
    let input = include_str!("input9.txt");
    let mut sum: i64 = 0;
    let mut sum2: i64 = 0;
    
    for line in input.trim().lines() {
        let mut rows: Vec<Vec<i64>> = Vec::new();
        let mut nums: Vec<i64> = Vec::new();
        for num in line.split(' ') {
            nums.push(num.parse::<i64>().unwrap());
            
        }
        rows.push(nums);

        loop {
            let mut zeros = true;
            let mut nums: Vec<i64> = Vec::new();

            for idx in 0..rows[rows.len()-1].len() - 1 {
                let num = rows[rows.len()-1][idx+1] - rows[rows.len()-1][idx];

                if num != 0 {
                    zeros = false;
                }
                nums.push(num);
            }
            rows.push(nums);

            if zeros {
                break;
            }
        }

        let s = rows.len();
        for i in 1..s {
            let idx = s - 1 - i;

            let res = rows[idx][rows[idx].len()-1] + rows[idx+1][rows[idx+1].len()-1];
            rows[idx].push(res);

            let res2 = rows[idx][0] - rows[idx+1][0];
            rows[idx].insert(0, res2);
        }

        sum += rows[0][rows[0].len()-1];
        sum2 += rows[0][0];
    }

   
    println!("{} {}", sum, sum2);
}
