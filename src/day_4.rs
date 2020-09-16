pub mod day4 {
    use std::fs;
    use itertools::Itertools;

    pub fn read_passphrases(filename: &str) -> String {
        let passphrases: String = fs::read_to_string(filename)
            .expect("Unable to read file.");
        passphrases
    }

    pub fn count_valid_passphrases(passphrases: &String) -> i32 {
        let mut sum = 0;
        for passphrase in passphrases.lines() {
            if valid_passphrase(passphrase) {
                sum += 1;
            }
        }
        sum
    }

    pub fn valid_passphrase(passphrase: &str) -> bool {
        for words in passphrase.split(' ').combinations(2) {
            if words[0] == words[1] {
                return false;
            }
        }
        true
    }

    pub fn count_valid_passphrases2(passphrases: &String) -> i32 {
        let mut sum = 0;
        for passphrase in passphrases.lines() {
            if valid_passphrase2(passphrase) {
                sum += 1;
            }
        }
        sum
    }

    pub fn valid_passphrase2(passphrase: &str) -> bool {
        for words in passphrase.split(' ').combinations(2) {
            let word_1: String = words[0].chars().sorted().collect();
            let word_2: String = words[1].chars().sorted().collect();
            if word_1 == word_2 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::day4::valid_passphrase;
    use super::day4::valid_passphrase2;

    #[test]
    fn test_valid_passphrase_1() {
        assert_eq!(valid_passphrase(&"aa bb cc dd ee"), true);
    }

    #[test]
    fn test_valid_passphrase_2() {
        assert_eq!(valid_passphrase(&"aa bb cc dd aa"), false);
    }

    #[test]
    fn test_valid_passphrase_3() {
        assert_eq!(valid_passphrase(&"aa bb cc dd aaa"), true);
    }

    #[test]
    fn test_valid_passphrase2_1() {
        assert_eq!(valid_passphrase2(&"abcde fghij"), true);
    }

    #[test]
    fn test_valid_passphrase2_2() {
        assert_eq!(valid_passphrase2(&"abcde xyz ecdab"), false);
    }

    #[test]
    fn test_valid_passphrase2_3() {
        assert_eq!(valid_passphrase2(&"a ab abc abd abf abj"), true);
    }

    #[test]
    fn test_valid_passphrase2_4() {
        assert_eq!(valid_passphrase2(&"iiii oiii ooii oooi oooo"), true);
    }

    #[test]
    fn test_valid_passphrase2_5() {
        assert_eq!(valid_passphrase2(&"oiii ioii iioi iiio"), false);
    }
}
