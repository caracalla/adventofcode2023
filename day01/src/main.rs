// Day 1
// Part 1
fn part1() -> i32 {
    let file_contents = std::fs::read_to_string("./input.txt").unwrap();

    let mut sum = 0;

    for line in file_contents.split('\n') {
        if line.len() > 0 {
            // println!("line: {}", line);

            let mut num: String = Default::default();

            for char in line.chars() {
                if char.is_numeric() {
                    num.push(char);
                    break;
                }
            }

            for char in line.chars().rev() {
                if char.is_numeric() {
                    num.push(char);
                    break;
                }
            }
            
            // println!("  num: {}", num);

            sum += num.parse::<i32>().unwrap();
        }
    }
    
    sum
}

// Part 2
fn part2() -> usize {
    0 // TODO
}

fn main() {
    println!("sum part 1: {}", part1());
    println!("sum part 2: {}", part2());
}
