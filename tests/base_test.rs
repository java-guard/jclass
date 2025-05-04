use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::{read, File};
use std::io::{BufReader, Cursor};
use std::time::Instant;
use jclass::jclass_info::JClassInfo;
use jclass::attribute_info::CodeAttribute;
use jclass::common::constants::CODE_TAG;
use jclass::constant_pool::ConstantValue;

#[test]
fn base_test() {
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
    let mut min_t = u128::MAX;
    let mut max_t = 0;
    let mut avg_t = 0;
    for _ in 0..10000 {
        // let content_ref = content.clone();
        // let cursor = Cursor::new(content_ref);
        let cursor = Cursor::new(&content);
        let now = Instant::now();
        let mut info = JClassInfo::from_reader(&mut cursor.into());
        if let Ok(mut info) = info {
            let constant_count = info.constant_pool.get_constant_count();
            let mut index_set = HashSet::with_capacity(5);
            for i in 0..constant_count {
                let value = info.constant_pool.get_constant_item(i);
                match value {
                    ConstantValue::ConstantString(utf8_index) => {
                        if let ConstantValue::ConstantUtf8(utf8_str) = info.constant_pool.get_constant_item(*utf8_index) {
                            if utf8_str == CODE_TAG {
                                index_set.insert(i);
                            }
                        }
                    }
                    ConstantValue::ConstantUtf8(utf8_str) => {
                        if utf8_str == CODE_TAG {
                            index_set.insert(i);
                        }
                    }
                    _ => {}
                }
            }
            for method_info in info.methods {
                let mut has_code = false;
                for attribute_info in method_info.attributes {
                    if index_set.contains(&attribute_info.name) {
                        if let Ok(attr) = CodeAttribute::new_with_data(&attribute_info.data) {
                            if attr.codes.len() <= 0 {
                                println!("{}", attr.codes.len());
                            }
                            has_code = true;
                        }
                    }
                }
                if !has_code && method_info.name != 161 {
                    println!("not found code");
                }
            }
        }
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

#[test]
fn test_parser() {
    let file_path = "D:\\data\\code\\idea\\test-all\\target\\classes\\cn\\kyle\\test\\all\\base\\HutoolScriptTest.class";
    let file_path = "D:\\data\\code\\project\\JavaGuard\\JavaGuard\\target\\classes\\javassist\\bytecode\\ClassDecryption.class";
    let content = File::open(file_path).unwrap();
    let now = Instant::now();
    let mut  info = JClassInfo::from_reader(&mut content.into());
    println!(">> {:?}", now.elapsed().as_nanos());
    if let Ok(info) = info {
        println!("{:?}", &info);
    }

    let content = read(file_path).unwrap();
    let mut t = 0;
    for _ in 0..10000 {
        // let content_ref = content.clone();
        // let cursor = Cursor::new(content_ref);
        let cursor = Cursor::new(&content);
        let now = Instant::now();
        let mut _info = JClassInfo::from_reader(&mut cursor.into());
        let duration = now.elapsed();
        t += duration.as_nanos();
    }
    println!(">> {:?}", t);
}