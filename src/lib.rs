use crate::class_parser::ClassParser;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

mod classfile_constants;
mod class_parser;
mod constant_pool;
mod error;
mod jclass_info;
mod util;
mod field_info;
mod attribute_info;
mod constants;
mod method_info;
mod support;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read;
    use std::io::{BufReader, Cursor};
    use std::time::Instant;

    #[test]
    fn test_parser() {
        let file_path = "D:\\data\\code\\idea\\test-all\\target\\classes\\cn\\kyle\\test\\all\\base\\HutoolScriptTest.class";
        let content = File::open(file_path).unwrap();
        let now = Instant::now();
        let mut  info = ClassParser::new(content.into());
        let _result = info.load_all().unwrap();
        println!(">> {:?}", now.elapsed().as_nanos());
        // println!("{:#?}", &info.get_jclass_info());

        let mut content = read(file_path).unwrap();
        let mut t = 0;
        for i in 0..10000 {
            // let content_ref = content.clone();
            // let cursor = Cursor::new(content_ref);
            let cursor = Cursor::new(&content);
            let now = Instant::now();
            let mut info = ClassParser::new(cursor.into());
            let _result = info.load_all();
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
        let mut  info = ClassParser::new(BufReader::new(content).into());

        let mut total = 0;
        let now = Instant::now();
        info.magic().unwrap();
        let duration = now.elapsed();
        total += duration.as_nanos();
        println!(">> magic: {:?}", duration);

        let now = Instant::now();
        info.minor_version().unwrap();
        let duration = now.elapsed();

        total += duration.as_nanos();
        println!(">> minor_version: {:?}", duration);

        let now = Instant::now();
        info.major_version().unwrap();
        let duration = now.elapsed();

        total += duration.as_nanos();
        println!(">> major_version: {:?}", duration);

        let now = Instant::now();
        info.constant_pool().unwrap();
        let duration = now.elapsed();

        total += duration.as_nanos();
        println!(">> constant_pool: {:?}", duration);

        let now = Instant::now();
        info.access_flags().unwrap();
        let duration = now.elapsed();

        total += duration.as_nanos();
        println!(">> access_flags: {:?}", duration);

        let now = Instant::now();
        info.class_index().unwrap();
        let duration = now.elapsed();

        total += duration.as_nanos();
        println!(">> class_index: {:?}", duration);

        let now = Instant::now();
        info.superclass_index().unwrap();
        let duration = now.elapsed();

        total += duration.as_nanos();
        println!(">> superclass_index: {:?}", duration);

        let now = Instant::now();
        info.interfaces().unwrap();
        let duration = now.elapsed();

        total += duration.as_nanos();
        println!(">> interfaces: {:?}", duration);

        let now = Instant::now();
        info.fields().unwrap();
        let duration = now.elapsed();

        total += duration.as_nanos();
        println!(">> fields: {:?}", duration);

        let now = Instant::now();
        info.methods().unwrap();
        let duration = now.elapsed();

        total += duration.as_nanos();
        println!(">> methods: {:?}", duration);

        let now = Instant::now();
        info.attributes().unwrap();
        let duration = now.elapsed();

        total += duration.as_nanos();
        println!(">> attributes: {:?}", duration);
        println!(">> total: {:?} ns", total);
    }
}

