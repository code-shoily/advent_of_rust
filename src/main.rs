mod year_15;
mod reader;

fn main() {
    let input = reader::read_input(15, 1);
    let (a, b) = year_15::day_1::run(input);
    println!("{} {}", a, b)
}
