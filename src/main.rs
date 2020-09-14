mod day_1;

pub use day_1::day1;

fn main() {
    let numbers = day1::numbers_from_file("input/day1.txt");
    println!("day 1 - part 1:\t{}", day1::captcha(&numbers));
    println!("day 1 - part 2:\t{}", day1::captcha2(&numbers));
}
