use std::io::Result;

// rust 构建脚本，构建 rust crate 之前的 Hook；
// 构建脚本必须在根目录，且文件名为 build.rs；
// 构建脚本需要的依赖项可以在 Cargo.toml 的 [build-dependencies] 中声明。
fn main() -> Result<()> {
    // 第一个参数指明 .proto 文件名，也可以带路径
    // 第二个参数指明 .proto 文件所在的目录
    // 生成的 rust 代码默认存放在 Cargo OUT_DIR 路径下，可以使用 include! 宏将其导入到我们的 rust 模块中，使用 env! 宏获取 OUT_DIR 的实际值。
    prost_build::compile_protos(&["rpc.proto"], &["src/"])?;
    Ok(())
}