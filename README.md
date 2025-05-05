# jclass

> a simple lib for java class file parse or edit
> 

* Example:
```rust
    // parse
    let content = File::open(file_path).unwrap();
    let class_info = JClassInfo::from_reader(&mut content.into());

    // to bytes
    let mut bytes = Vec::new();
    {
        let mut writer = BufWriter::new( & mut bytes).into();
        info.write_to( & mut writer).unwrap();
    }
```