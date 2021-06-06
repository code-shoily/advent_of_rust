use std::collections::HashSet;

fn process(raw_data: &str) -> Vec<i32> {
    raw_data.lines().map(|i| {
        let as_num: i32 = i.trim().parse().expect("Not a number");
        as_num
    }).collect()
}

fn run_1(data: &str) -> Result<i32, &str> {
    let numbers: HashSet<i32> = process(data).iter().cloned().collect();

    for number in &numbers {
        let remaining = 2020 - number;
        if numbers.contains(&remaining) {
            return Ok(remaining * number);
        }
    }
    Err("Number not found, perhaps invalid input-set?")
}

fn run_2(data: &str) -> Result<i32, &str> {
    let numbers: HashSet<i32> = process(data).iter().cloned().collect();

    for number in &numbers {
        let remaining_1 = 2020 - number;
        for next_number in &numbers {
            let remaining_2 = remaining_1 - next_number;
            if numbers.contains(&remaining_2) {
                return Ok(number * next_number * remaining_2);
            }
        }
    }
    Err("Number not found, perhaps invalid input-set?")
}

#[allow(dead_code)]
pub fn run(data: String) -> (i32, i32) {
    (run_1(&data).unwrap(), run_2(&data).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_fn() {
        let raw_data = "1721\n979\n366\n299\n675\n1456";
        let ints = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(process(raw_data), ints)
    }

    #[test]
    fn test_run_1_fn() {
        let raw_data = "1721\n979\n366\n299\n675\n1456";
        assert_eq!(run_1(raw_data).unwrap(), 514579)
    }

    #[test]
    fn test_run_2_fn() {
        let raw_data = "1721\n979\n366\n299\n675\n1456";
        assert_eq!(run_2(raw_data).unwrap(), 241861950)
    }
}