mod year_20;
mod reader;

fn main() {
    let input = reader::read_input(20, 1);
    let (a, b) = year_20::day_1::run(input);
    println!("{} {}", a, b)
}
