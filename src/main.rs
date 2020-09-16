mod day_1;
mod day_2;
mod day_4;

use day_1::day1;
use day_2::day2;
use day_4::day4;

fn main() {
    let numbers = day1::numbers_from_file("input/day1.txt");
    println!("day 1 - part 1:\t{}", day1::captcha(&numbers));
    println!("day 1 - part 2:\t{}", day1::captcha2(&numbers));

    let array = day2::read_csv();
    println!("day 2 - part 1:\t{}", day2::checksum(&array));
    println!("day 2 - part 1:\t{}", day2::checksum2(&array));

    let abc = day4::read_passphrases("input/day4.txt");
    println!("day 4 - part 1:\t{}", day4::count_valid_passphrases(&abc));
    println!("day 4 - part 2:\t{}", day4::count_valid_passphrases2(&abc));
}
