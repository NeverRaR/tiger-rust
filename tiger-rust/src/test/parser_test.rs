use std::{
    collections::HashSet,
    fs::{self, metadata},
};

use super::util::all_path;
use crate::parser::parser::compile;

const TEST_DATA_PATH: &str = "./src/test/testcases/tigcases/";

fn build_err_case_set() -> HashSet<String> {
    let mut case_set = HashSet::<String>::new();
    let case_path = "./src/test/testcases/parser_error_case.json";
    let case_json = fs::read_to_string(case_path)
        .expect(format!("Something went wrong reading {}", case_path).as_str());
    let case_list = serde_json::from_str::<Vec<String>>(&case_json).unwrap();
    for case in case_list {
        case_set.insert(case);
    }
    case_set
}

#[test]
fn test_parser() {
    let err_cases = build_err_case_set();
    let input_paths = all_path(TEST_DATA_PATH).unwrap();

    for (_, input_path) in input_paths.into_iter().enumerate() {
        if metadata(&input_path).unwrap().is_dir() {
            continue;
        }
        let input = fs::read_to_string(&input_path)
            .expect(format!("Something went wrong reading {}", input_path).as_str());
        println!("{}", input_path);
        println!("{}", input);
        let result = compile(&input);
        let file_name = input_path.strip_prefix(TEST_DATA_PATH).unwrap();
        if err_cases.contains(file_name) {
            assert!(result.is_err())
        } else {
            assert!(result.is_ok())
        }
    }
}

#[test]
fn test_my_data() {
    let my_data_path = "./src/test/testcases/tigcases/mydata.tig";
    let input = fs::read_to_string(&my_data_path)
        .expect(format!("Something went wrong reading {}", my_data_path).as_str());
    println!("{}", my_data_path);
    println!("{}", input);
    let result = compile(&input);

    println!("{}", result.unwrap().to_yaml_string().unwrap());
}
