#![allow(unused_attributes, unused, non_upper_case_globals)]

mod common;
mod classfile_constants;
mod constant_pool;
mod jclass_info;
mod util;
mod field_info;
mod attribute_info;
mod method_info;
mod lazy_value;
mod support;

use std::cmp::{max, min};
use std::fs::{read, File};
use std::io::{BufReader, Cursor};
use std::time::Instant;
use crate::jclass_info::JClassInfo;

fn main() {
    let file_path = "D:\\data\\code\\idea\\test-all\\target\\classes\\cn\\kyle\\test\\all\\base\\HutoolScriptTest.class";
    let file_path = "D:\\data\\code\\project\\JavaGuard\\JavaGuard\\target\\classes\\javassist\\bytecode\\ClassDecryption.class";
    let content = File::open(file_path).unwrap();
    let now = Instant::now();
    let mut  info = JClassInfo::from_reader(&mut content.into());
    println!(">> {:?}", now.elapsed().as_nanos());
    if let Ok(inf) = info {
        // println!("{:?}", &info);
    }

    let content = read(file_path).unwrap();
    let mut t = 0;
    let mut min_t = 999999999999;
    let mut max_t = 0;
    let mut avg_t = 0;
    for _ in 0..10000 {
        // let content_ref = content.clone();
        // let cursor = Cursor::new(content_ref);
        let cursor = Cursor::new(&content);
        let now = Instant::now();
        let mut _info = JClassInfo::from_reader(&mut cursor.into());
        let duration = now.elapsed();
        let n = duration.as_nanos();
        t += n;
        min_t = min(n, min_t);
        max_t = max(n, max_t);
        avg_t += n;

    }
    println!(">> {:?}", t);
    println!(">> min: {:?}", min_t);
    println!(">> max: {:?}", max_t);
    println!(">> avg: {:?}", avg_t/10000);
}