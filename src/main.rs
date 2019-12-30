use serde_json::Value;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const KEY: &str = "PCC.MINI";

fn tohex(h: u8, l: u8) -> u8 {
    let x = |x| match x {
        b'0'..=b'9' => x - 48,
        b'A'..=b'F' | b'a'..=b'f' => (x - b'a') + 10,
        _ => 0,
    };
    return x(h) << 4 | x(l);
}

fn decode(p: &str) -> Vec<u8> {
    let bs = p.as_bytes();
    let mut v: Vec<u8> = Vec::new();
    for i in (0..bs.len()).step_by(2) {
        v.push(tohex(bs[i] as u8, bs[i + 1] as u8));
    }
    v
}

fn init_key() -> Vec<u8> {
    let mut rs = Vec::<u8>::with_capacity(256);
    let b_key = KEY.as_bytes();

    let mut index2: usize = 0;
    let mut index1: usize = 0;
    for i in 0..rs.capacity() {
        rs.push(i as u8);
    }
    for i in 0..rs.capacity() {
        index2 = (((b_key[index1] as usize) + (rs[i] as usize) + (index2)) & 0xff) as usize;
        let tmp = rs[i];
        rs[i] = rs[index2];
        rs[index2] = tmp;
        index1 = (index1 + 1) % b_key.len();
    }
    rs
}

fn rc_base(input: &Vec<u8>) -> String {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut key = init_key();
    let mut result = Vec::<u8>::with_capacity(input.len());
    if key.len() > 0 {
        let mut xor_index: usize;
        for i in 0..input.len() {
            x = (x + 1) as u8 as usize;
            y = (((key[x] as usize) + y) & 0xff) as usize;
            let tmp = key[x];
            key[x] = key[y];
            key[y] = tmp;
            xor_index = (((key[x] as usize) + (key[y] as usize)) & 0xff) as usize;
            result.push(input[i] ^ key[xor_index]);
        }
    }
    return String::from_utf8(result).unwrap();
}

#[inline]
fn process(reader: &mut dyn BufRead) {
    let mut _line = String::new();
    while reader.read_line(&mut _line).unwrap_or(0) > 0 {
        let v = decode(&_line.trim_matches(|x| x == '\n' || x == '\r'));
        let json_str = rc_base(&v);
        let _v: Value = match serde_json::from_str(&json_str) {
            Ok(v) => v,
            _ => {
                println!("error json string: {}", json_str);
                continue;
            }
        };
        let _pretty = match serde_json::to_string_pretty(&_v) {
            Ok(v) => v,
            _ => continue,
        };
        println!("{}", _pretty);
        _line.clear();
    }
}

fn main() {
    let input = env::args().nth(1);
    if let Some(filename) = input {
        process(&mut BufReader::new(File::open(filename).unwrap()));
    } else {
        process(&mut BufReader::new(io::stdin()));
    }
    
}
