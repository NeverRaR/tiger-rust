use std::fs::{self, metadata};

use super::util::all_path;
use crate::lexer::lexer::tokenize;

const TEST_DATA_PATH: &str = "./src/test/testcases";

#[test]
fn test_lexer() {
    let input_paths = all_path(TEST_DATA_PATH).unwrap();
    for (_, input_path) in input_paths.into_iter().enumerate() {
        println!("{}", input_path);
        if metadata(&input_path).unwrap().is_dir() {
            continue;
        }
        let input = fs::read_to_string(&input_path)
            .expect(format!("Something went wrong reading {}", input_path).as_str());
        println!("{}", input);
        for (_, token) in tokenize(&input).into_iter().enumerate() {
            println!("{}", token)
        }
    }
}
