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

#[cfg(test)]
mod tests {
    use std::fs::{read, File};
    use std::io::{BufReader, Cursor};
    use std::time::Instant;
    use crate::jclass_info::JClassInfo;

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

    #[test]
    fn test_parser_step() {
        // let file_path = "D:\\data\\code\\idea\\test-all\\target\\classes\\cn\\kyle\\test\\all\\base\\HutoolScriptTest.class";
        let file_path = "D:\\data\\code\\project\\JavaGuard\\JavaGuard\\target\\classes\\javassist\\bytecode\\ClassDecryption.class";
        let content = File::open(file_path).unwrap();

        let mut total = 0;
        let now = Instant::now();
        let mut  info = JClassInfo::from_reader(&mut BufReader::new(content).into());
        let duration = now.elapsed();
        total += duration.as_nanos();
        println!(">> total: {:?}", duration);
    }
}

