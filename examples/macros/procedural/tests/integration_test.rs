use procedural::{query, RawBuilder};

#[test]
fn query_should_work() {
    // 这个过程宏定义了一个 hello() 函数
    query!(SELECT * FROM users WHERE age > 10);
    // 调用 hello() 函数
    hello();
}


#[test]
fn raw_builder_should_work() {
    #[derive(Debug, RawBuilder)]
    #[allow(dead_code)]
    struct Command {
        executable: String,
        args: Vec<String>,
        env: Vec<String>,
        current_dir: Option<String>,
    }

    let command = Command::builder()
        .executable(String::from("cargo"))
        .args(vec![String::from("build"), String::from("--release")])
        .env(vec![])
        .build()
        .unwrap();
    assert!(command.current_dir.is_none());

    let command = Command::builder()
        .executable(String::from("cargo"))
        .args(vec![String::from("build"), String::from("--release")])
        .env(vec![])
        .current_dir(String::from(".."))
        .build()
        .unwrap();
    assert!(command.current_dir.is_some());

    println!("{:?}", command);
}