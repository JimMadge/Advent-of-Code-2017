pub mod day2 {
    // use std::error::Error;

    // pub fn read_csv() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    //     let mut array: Vec<Vec<i32>> = Vec::new();

    //     let mut rdr = csv::ReaderBuilder::new()
    //         .delimiter(b'\t')
    //         .has_headers(false)
    //         .from_path("input/day2.txt")?;
    //     for result in rdr.records() {
    //         let record = result?;

    //         let mut row: Vec<i32> = Vec::new();
    //         for i in record.iter() {
    //             row.push(i.parse().unwrap());
    //         }
    //         array.push(row);
    //     }
    //     Ok(array)
    // }

    pub fn read_csv() -> Vec<Vec<i32>> {
        let mut array: Vec<Vec<i32>> = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_path("input/day2.txt")
            .expect("Unable to read file");
        for result in rdr.records() {
            let record = result.unwrap();

            let mut row: Vec<i32> = Vec::new();
            for i in record.iter() {
                row.push(i.parse().unwrap());
            }
            array.push(row);
        }
        array
    }

    pub fn checksum(array: Vec<Vec<i32>>) -> i32 {
        let mut checksum = 0;

        for row in array {
            let max_val = row.iter().max().unwrap();
            let min_val = row.iter().min().unwrap();
            checksum += max_val - min_val;
        }
        checksum
    }
}

#[cfg(test)]
mod tests{
    use super::day2::checksum;

    #[test]
    fn test_checksum() {
        let a = vec![vec![5,1,9,5], vec![7,5,3], vec![2,4,6,8]];
        assert_eq!(checksum(a), 18)
    }
}
