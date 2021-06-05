fn process(data: &String) -> Vec<char> {
    data.chars().collect::<Vec<char>>()
}

fn step(current_floor: i16, instruction: &char) -> i16 {
    match instruction {
        '(' => current_floor + 1,
        ')' => current_floor - 1,
        _ => panic!("invalid input")
    }
}

fn run_1(data: &String) -> i16 {
    process(data).iter().fold(0, step)
}

fn run_2(data: &String) -> i16 {
    let mut index = 0;
    let mut current_floor = 0;

    for instruction in process(data).iter() {
        index += 1;
        current_floor = step(current_floor, &instruction);
        if current_floor == -1 {
            break
        }
    }

    index
}

pub fn run(data: String) -> (i16, i16) {
    (run_1(&data), run_2(&data))
}