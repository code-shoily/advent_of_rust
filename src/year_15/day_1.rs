fn process(data: &str) -> Vec<char> {
    data.chars().collect::<Vec<char>>()
}

fn step(current_floor: i16, instruction: &char) -> i16 {
    match instruction {
        '(' => current_floor + 1,
        ')' => current_floor - 1,
        _ => panic!("invalid input")
    }
}

fn run_1(data: &str) -> i16 {
    process(data).iter().fold(0, step)
}

fn run_2(data: &str) -> i16 {
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

#[allow(dead_code)]
pub fn run(data: String) -> (i16, i16) {
    (run_1(&data), run_2(&data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_fn() {
        assert_eq!(process("(())"), ['(', '(', ')', ')'])
    }

    #[test]
    fn test_run_1() {
        assert_eq!(run_1("(())"), 0);
        assert_eq!(run_1("((("), 3);
        assert_eq!(run_1("(()(()("), 3);
        assert_eq!(run_1("(()(()("), 3);
        assert_eq!(run_1("))((((("), 3);
        assert_eq!(run_1("())"), -1);
        assert_eq!(run_1("))("), -1);
        assert_eq!(run_1(")))"), -3);
        assert_eq!(run_1(")())())"), -3);
    }

    #[test]
    fn test_run_2() {
        assert_eq!(run_2(")"), 1);
        assert_eq!(run_2("()())"), 5);
    }
}