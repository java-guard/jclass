use std::fs::File;
use crate::class_parser::ClassParser;

mod classfile_constants;
mod class_parser;
mod constant_pool;
mod common;
mod jclass_info;
mod util;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let content = File::open("D:\\data\\code\\idea\\test-all\\target\\classes\\cn\\kyle\\test\\all\\base\\HutoolScriptTest.class").unwrap();
        let mut  info = ClassParser::new(content);
        let result = info.major_version().unwrap();
        // let result = info.magic().unwrap();
        println!("{:#?}", &info.get_jclass_info());
        // let result = info.minor_version().unwrap();
        // println!("{result}");
        // let result = info.major_version().unwrap();
        // println!("{result}");
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

