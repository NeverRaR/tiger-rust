use std::sync::Mutex;

use serde::{Deserialize, Serialize};

lazy_static! {
    static ref LINE_POS: Mutex<Vec<usize>> = Mutex::new(Vec::new());
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PosInfo {
    pub line: usize,
    pub col: usize,
}

pub fn parse_line_pos(s: &str) {
    let mut line_pos = LINE_POS.lock().unwrap();
    line_pos.push(0);
    let byte = s.as_bytes();
    let len = byte.len();
    let mut i = 0;
    while i < len {
        let c = *byte.get(i).unwrap();
        if c as char == '\r' {
            if i < len - 1 {
                let c_next = *byte.get(i + 1).unwrap();
                if c_next as char == '\n' {
                    i += 1
                }
                line_pos.push(i + 1);
            }
        } else if c as char == '\n' {
            line_pos.push(i + 1);
        }
        i += 1;
    }
}

pub fn get_position_info(pos: usize) -> PosInfo {
    let line_pos = LINE_POS.lock().unwrap();
    let pos_index = line_pos.partition_point(|x| *x < pos);
    let len_start_pos = match line_pos.get(pos_index) {
        Some(pos) => pos.clone(),
        None => 0 as usize,
    };
    PosInfo {
        line: pos_index,
        col: if len_start_pos == 0 {
            pos
        } else {
            len_start_pos - pos
        },
    }
}

pub fn emit_error(pos: usize, msg: &str) {
    let info = get_position_info(pos);
    println!("{}.{}:{}", info.line, info.col, msg)
}

pub fn reset_line_pos() {
    LINE_POS.lock().unwrap().clear();
}
