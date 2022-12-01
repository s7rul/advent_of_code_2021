use advent_of_code_2022::*;

fn main() {
    let input = read_input_to_vec("input/day1.txt");


    let mut tmp: u32 = 0;

    let mut max1: u32 = 0;
    let mut max2: u32 = 0;
    let mut max3: u32 = 0;
    for l in input {
        if l == "" {
            if tmp >= max1 {
                max3 = max2;
                max2 = max1;
                max1 = tmp;
            } else if tmp >= max2 {
                max3 = max2;
                max2 = tmp;
            } else if tmp > max3 {
                max3 = tmp;
            }
            tmp = 0;
        } else {
            tmp += l.parse::<u32>().unwrap();
        }
    }
    if tmp > max1 {
        max3 = max2;
        max2 = max1;
        max1 = tmp;
    } else if tmp > max2 {
        max3 = max2;
        max2 = tmp;
    } else if tmp > max3 {
        max3 = tmp;
    }


    println!("Max: {}", max1 + max2 + max3);
}