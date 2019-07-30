use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn load_query(path: &str, limit: u32) -> Vec<(i64, i64)> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut ret = Vec::new();
    let mut cnt = 0;
    reader.lines().for_each(|line| {
        if cnt > 0 && cnt >= limit {
            return
        }
        let data = line.unwrap();
        let tmp: Vec<&str> = data.split(",").collect();
        let u = tmp[0].parse::<i64>().unwrap();
        let v = tmp[1].parse::<i64>().unwrap();
        ret.push((u, v));
        cnt += 1;
    });
    ret
}

pub fn load_query2(path: &str, limit: u32) -> Vec<(i64, i64)> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let mut ret = Vec::new();
    let mut cnt = 0;
    reader.lines().for_each(|line| {
        if cnt > 0 && cnt >= limit {
            return
        }
        let data = line.unwrap();
        let tmp: Vec<&str> = data.split_whitespace().collect();
        let u = tmp[0].parse::<i64>().unwrap();
        let v = tmp[1].parse::<i64>().unwrap();
        ret.push((u, v));
        cnt += 1;
    });
    ret
}
