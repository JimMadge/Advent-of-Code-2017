pub mod day1 {
    use std::fs;

    pub fn numbers_from_file(filename: &str) -> Vec<u32> {
        let input = fs::read_to_string(filename)
            .expect("Unable to read the file");

        let mut numbers: Vec<u32> = Vec::new();
        for i in input.trim().chars(){
            numbers.push(i.to_digit(10).unwrap());
        }

        numbers
    }

    pub fn captcha(numbers: &Vec<u32>) -> u32 {
        let length = numbers.len();

        let mut sum: u32 = 0;
        for i in 0..length {
            if numbers[i] == numbers[(i+1)%length] {
                sum += numbers[i];
            }
        }

        sum
    }

    pub fn captcha2(numbers: &Vec<u32>) -> u32 {
        let length = numbers.len();
        let stride = length/2;

        let mut sum: u32 = 0;
        for i in 0..length {
            if numbers[i] == numbers[(i+stride)%length] {
                sum += numbers[i];
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests{
    use super::day1::captcha;
    use super::day1::captcha2;

    #[test]
    fn test_captcha_1() {
        assert_eq!(captcha(&vec![1,1,2,2]), 3);
    }

    #[test]
    fn test_captcha_2() {
        assert_eq!(captcha(&vec![1,1,1,1]), 4);
    }

    #[test]
    fn test_captcha_3() {
        assert_eq!(captcha(&vec![1,2,3,4]), 0);
    }

    #[test]
    fn test_captcha_4() {
        assert_eq!(captcha(&vec![9,1,2,1,2,1,2,9]), 9);
    }

    #[test]
    fn test_captcha2_1() {
        assert_eq!(captcha2(&vec![1,2,1,2]), 6);
    }

    #[test]
    fn test_captcha2_2() {
        assert_eq!(captcha2(&vec![1,2,2,1]), 0);
    }

    #[test]
    fn test_captcha2_3() {
        assert_eq!(captcha2(&vec![1,2,1,2]), 6);
    }

    #[test]
    fn test_captcha2_4() {
        assert_eq!(captcha2(&vec![1,2,3,4,2,5]), 4);
    }

    #[test]
    fn test_captcha2_5() {
        assert_eq!(captcha2(&vec![1,2,3,1,2,3]), 12);
    }

    #[test]
    fn test_captcha2_6() {
        assert_eq!(captcha2(&vec![1,2,1,3,1,4,1,5]), 4);
    }
}
