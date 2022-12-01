use std::fs;

fn main() {
    //println!("{}", max_cal("data/in.txt"))
    println!("{}", top_three_cal("data/in.txt"))
}

fn max_cal(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    //println!("Content:\n{contents}");
    let calories = get_calories_from_input(contents);
    //println!("calories={:?}", calories);
    let max: i32 = calories
        .into_iter()
        .map(|cal| cal.into_iter().sum())
        .max()
        .unwrap();
    return max;
}

fn get_calories_from_input(content: String) -> Vec<Vec<i32>> {
    let calories: Vec<Vec<i32>> = content
        .split("\n\n")
        .into_iter()
        .map(|cals| {
            let cals_array: Vec<i32> = cals
                .split("\n")
                .filter_map(|cal| cal.parse().ok())
                .collect();
            cals_array
        })
        .collect();
    calories
}

fn top_three_cal(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    //println!("Content:\n{contents}");
    let calories = get_calories_from_input(contents);
    //println!("calories={:?}", calories);
    let mut max: Vec<i32> = calories.iter().map(|cal| cal.into_iter().sum()).collect();
    max.sort();
    let rev: Vec<i32> = max.into_iter().rev().collect();
    let top_three = rev.iter().take(3);
    return top_three.sum();
}

#[cfg(test)]
mod tests {
    use crate::max_cal;
    use crate::top_three_cal;

    #[test]
    fn test_part1_example() {
        assert_eq!(max_cal("data/in_example1.txt"), 24000);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(top_three_cal("data/in_example1.txt"), 45000);
    }
}
