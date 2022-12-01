use advent_of_code_2022::*;

fn main() {
    let input = read_input_to_vec("input/day1.txt");

    let mut sum: u32 = 0;
    let mut last: u32 = 0;

    let mut flag: bool = false;

    let mut window: [u32; 3] = [0; 3];
    let mut start: u32 = 0;

    for l in input {
        if l == "" {
        } else if flag {
            let current = l.parse::<u32>().unwrap();

            window[0] = window[1];
            window[1] = window[2];
            window[2] = current;

            let mut win_sum = 0;
            for n in window {
                win_sum += n;
            }

            if last < win_sum {
                println!("increase");
                sum += 1;
            } else {
                println!("no increase");
            }
            last = win_sum;


        } else {
            let current = l.parse::<u32>().unwrap();
            window[start as usize] = current;

            let mut win_sum = 0;
            for n in window {
                win_sum += n;
            }

            start += 1;
            println!("setup");
            if start == 3 {
                last = win_sum;

                flag = true;
            }
        }
    }

    println!("{}", sum);
}