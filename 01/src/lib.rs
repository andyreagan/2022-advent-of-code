#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let contents = "10
20

15
6

18";

        assert_eq!(30, search(contents));
    }
}

pub fn search<'a>(contents: &'a str) -> i32 {
    let mut max = 0;
    let mut tmp = 0;
    for line in contents.lines() {
        if line == "" {
            if tmp > max {
                max = tmp;
            }
            tmp = 0;
        } else {
            tmp = tmp + line.parse::<i32>().unwrap();
        }
    }

    max
}