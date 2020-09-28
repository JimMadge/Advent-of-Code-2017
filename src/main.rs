mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

use day_1::day1;
use day_2::day2;
use day_3::day3;
use day_4::day4;
use day_5::day5;

fn main() {
    let numbers = day1::numbers_from_file("input/day1.txt");
    println!("day 1 - part 1:\t{}", day1::captcha(&numbers));
    println!("day 1 - part 2:\t{}", day1::captcha2(&numbers));

    let array = day2::read_csv();
    println!("day 2 - part 1:\t{}", day2::checksum(&array));
    println!("day 2 - part 2:\t{}", day2::checksum2(&array));

    let address: u32 = 277678;
    println!("day 3 - part 1:\t{}",
        day3::manhattan_distance(day3::spiral_coordinates(address))
    );

    let passphrases = day4::read_passphrases("input/day4.txt");
    println!(
        "day 4 - part 1:\t{}",
        day4::count_valid_passphrases(&passphrases, day4::valid_passphrase)
        );
    println!(
        "day 4 - part 2:\t{}",
        day4::count_valid_passphrases(&passphrases, day4::valid_passphrase2)
        );

    let instructions = day5::read_instructions("input/day5.txt");
    println!("day 5 - part 1:\t{}", day5::count_steps(instructions));
    let instructions = day5::read_instructions("input/day5.txt");
    println!("day 5 - part 1:\t{}", day5::count_steps2(instructions));
}
