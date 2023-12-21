fn parse_input() -> Vec<Vec<isize>> {
    let file_contents = std::fs::read_to_string("./input.txt").unwrap();

    file_contents.split('\n').map(|line| {
        if line.len() == 0 {
            vec![]
        } else {
            line.split(' ').map(|num| num.parse::<isize>().unwrap()).collect()
        }
    }).collect()
}

fn is_zeroes(nums: &Vec<isize>) -> bool {
    for num in nums {
        if *num != 0 {
            return false;
        }
    }

    true
}

fn get_steps(nums: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut steps = vec![nums.clone()];

    while !is_zeroes(&steps[steps.len() - 1]) {
        let mut new_step = vec![];
        let last_step = &steps[steps.len() - 1];

        for i in 0..(last_step.len() - 1) {
            new_step.push(last_step[i + 1] - last_step[i]);
        }

        steps.push(new_step);
    }
    
    steps
}

fn get_next(nums: &Vec<isize>) -> isize {
    if nums.len() == 0 {
        return 0;
    }

    let steps = get_steps(nums);
    let mut result = 0;

    for step in steps.into_iter().rev() {
        result += step[step.len() - 1];
    }

    result
}

fn part1(input: &Vec<Vec<isize>>) -> isize {
    input.iter().map(|nums| get_next(nums)).sum()
}

fn get_previous(nums: &Vec<isize>) -> isize {
    if nums.len() == 0 {
        return 0;
    }

    let steps = get_steps(nums);
    let mut result = 0;

    for step in steps.into_iter().rev() {
        result -= step[0];
        result = -result;
    }

    result
}

fn part2(input: &Vec<Vec<isize>>) -> isize {
    input.iter().map(|nums| get_previous(nums)).sum()
}

fn main() {
    let input = parse_input();
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}
