mod day_1;
mod day_2;

use day_1::day1;
use day_2::day2;

fn main() {
    let numbers = day1::numbers_from_file("input/day1.txt");
    println!("day 1 - part 1:\t{}", day1::captcha(&numbers));
    println!("day 1 - part 2:\t{}", day1::captcha2(&numbers));

    let array = day2::read_csv();
    println!("day 2 - part 1:\t{}", day2::checksum(&array));
    println!("day 2 - part 1:\t{}", day2::checksum2(&array));
}
