use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub fn read_points(file_name: &str) -> (Vec<f64>, Vec<f64>) {
    let error_message = format!("something went wrong reading file {}", file_name);
    let contents = fs::read_to_string(file_name).expect(&error_message);

    let mut xs: Vec<f64> = Vec::new();
    let mut ys: Vec<f64> = Vec::new();

    for line in contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        xs.push(words[0].parse().unwrap());
        ys.push(words[1].parse().unwrap());
    }

    return (xs, ys);
}

pub fn read_vector<T: FromStr>(file_name: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let error_message = format!("something went wrong reading file {}", file_name);
    let contents = fs::read_to_string(file_name).expect(&error_message);
    let v = contents.lines().map(|s| s.parse().unwrap()).collect();

    return v;
}
