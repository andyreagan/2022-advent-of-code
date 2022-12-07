#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let contents = "10
20

15
7

1

18

";

        assert_eq!(30, search(contents));
        assert_eq!(70, top3(contents));
    }
}


pub fn search(contents: &str) -> i32 {
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


pub fn top3(contents: &str) -> i32 {
    let mut max = [0, 0, 0];
    let mut tmp = 0;
    for line in contents.lines() {
        if line == "" {
            for i in 1..3 {
                if tmp > max[i-1] {
                    max[i-1] = tmp;
                    max.sort();
                    break;
                }
            }
            tmp = 0;
        } else {
            tmp = tmp + line.parse::<i32>().unwrap();
        }
    }

    // println!("With maxes:\n{max}");

    max.iter().sum()
}