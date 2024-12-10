use std::fs::File;
use crate::class_parser::ClassParser;

mod classfile_constants;
mod class_parser;
mod constant_pool;
mod common;
mod jclass_info;
mod util;
mod field_info;
mod attribute_info;
mod constants;
mod method_info;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::fs::read;
    use std::io::{BufReader, Cursor};
    use std::time::Instant;
    use super::*;

    #[test]
    fn test_parser() {
        let file_path = "D:\\data\\code\\idea\\test-all\\target\\classes\\cn\\kyle\\test\\all\\base\\HutoolScriptTest.class";
        let content = File::open(file_path).unwrap();
        let now = Instant::now();
        let mut  info = ClassParser::new(content);
        let _result = info.load_all().unwrap();
        println!(">> {:?}", now.elapsed().as_nanos());
        // println!("{:#?}", &info.get_jclass_info());

        let content = read(file_path).unwrap();
        let mut t = 0;
        for i in 0..10000 {
            let content_ref = content.clone();
            let cursor = Cursor::new(content_ref);
            let now = Instant::now();
            let mut info = ClassParser::new(cursor);
            let _result = info.load_all();
            let duration = now.elapsed();
            t += duration.as_nanos();
        }
        println!(">> {:?}", t)
    }

    #[test]
    fn test_parser_step() {
        let file_path = "D:\\data\\code\\idea\\test-all\\target\\classes\\cn\\kyle\\test\\all\\base\\HutoolScriptTest.class";
        let content = File::open(file_path).unwrap();
        let mut  info = ClassParser::new(BufReader::new(content));

        let now = Instant::now();
        info.magic().unwrap();
        let duration = now.elapsed();
        println!(">> magic: {:?}", duration);

        let now = Instant::now();
        info.minor_version().unwrap();
        let duration = now.elapsed();
        println!(">> minor_version: {:?}", duration);

        let now = Instant::now();
        info.major_version().unwrap();
        let duration = now.elapsed();
        println!(">> major_version: {:?}", duration);

        let now = Instant::now();
        info.constant_pool().unwrap();
        let duration = now.elapsed();
        println!(">> constant_pool: {:?}", duration);

        let now = Instant::now();
        info.access_flags().unwrap();
        let duration = now.elapsed();
        println!(">> access_flags: {:?}", duration);

        let now = Instant::now();
        info.class_index().unwrap();
        let duration = now.elapsed();
        println!(">> class_index: {:?}", duration);

        let now = Instant::now();
        info.superclass_index().unwrap();
        let duration = now.elapsed();
        println!(">> superclass_index: {:?}", duration);

        let now = Instant::now();
        info.interfaces().unwrap();
        let duration = now.elapsed();
        println!(">> interfaces: {:?}", duration);

        let now = Instant::now();
        info.fields().unwrap();
        let duration = now.elapsed();
        println!(">> fields: {:?}", duration);

        let now = Instant::now();
        info.methods().unwrap();
        let duration = now.elapsed();
        println!(">> methods: {:?}", duration);

        let now = Instant::now();
        info.attributes().unwrap();
        let duration = now.elapsed();
        println!(">> attributes: {:?}", duration);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

