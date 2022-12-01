use advent_of_code_2022::*;

fn main() {
    let input = read_input_to_vec("input/day2.txt");

    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    let mut x: i32 = 0;

    for l in input {
        let command: Vec<&str> = l.split(' ').collect();

        match command[0] {
            "forward" => {
                println!("forward");
                let amount = command[1].parse::<i32>().unwrap();
                x += amount;
                depth += amount * aim;
            }
            "up" => {
                println!("up");
                aim -= command[1].parse::<i32>().unwrap();
            }
            "down" => {
                println!("down");
                aim += command[1].parse::<i32>().unwrap();
            }
            _ => {
                println!("unsuported")
            }
        }
    }

    let result = x * depth;
    println!("{}", result);
}