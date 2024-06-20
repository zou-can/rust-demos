/// 过程宏示例
///
/// 过程宏必须定义在独立的 crate 中，需要在 _Cargo.toml_ 中增加
/// ```toml
/// [lib]
/// proc-macro = true
/// ```
///
/// 声明宏 *不能* 在过程宏的 crate 中导出。


use proc_macro::{TokenStream, TokenTree};
use std::collections::VecDeque;

use anyhow::Result;
use askama::Template;

/// 类函数宏
///
/// TokenStream 是一个 Iterator，里面包含一系列的
/// [TokenTree](https://doc.rust-lang.org/proc_macro/enum.TokenTree.html)
///
/// TokenTree 是一个枚举类型，包含以下值：
/// Ident（标识符）、Punct（标点符号）， Literal（字面量）和 Group（组）。
/// 这里的 Group，代表 {} [] <> () 中的内容。
#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);

    // TokenStream 实现了 FromStr Trait，因此可以将字符串直接解析成 TokenStream
    "fn hello() { println!(\"Hello world!\"); }"
        .parse()
        .unwrap()
}

/// derive 宏
///
/// 使用 `proce_macro_derive(派生宏名称)` 声明派生宏。
///
#[proc_macro_derive(RawBuilder)]
pub fn derive_raw_builder(input: TokenStream) -> TokenStream {
    BuilderContext::render(input).unwrap().parse().unwrap()
}


#[derive(Default, Debug)]
struct Fd {
    name: String,
    ty: String,
    optional: bool,
}

impl Fd {
    /// name 和 field 都是通过冒号 Punct 切分出来的 TokenTree 切片
    pub fn new(name: &[TokenTree], ty: &[TokenTree]) -> Self {

        // 把 TokenStream 切片转换成字符串数组
        let ty = ty.iter()
            .map(|v| match v {
                TokenTree::Ident(n) => n.to_string(),
                TokenTree::Punct(p) => p.as_char().to_string(),
                other => panic!("Expect ident, got {:?}", other),
            })
            .collect::<Vec<_>>();

        match name.last() {
            Some(TokenTree::Ident(name)) => {
                // 确定字段类型
                let (ty, optional) = if ty[0].as_str() == "Option" {
                    (&ty[2..ty.len() - 1], true)
                } else {
                    (&ty[..], false)
                };
                // 返回 Fd 实例
                Self {
                    name: name.to_string(),
                    ty: ty.join(""),
                    optional,
                }
            }
            other => panic!("Expect ident, got {:?}", other),
        }
    }
}

// 处理 jinjia 模板
#[derive(Template)]
#[template(path = "builder.j2", escape = "none")]
struct BuilderContext {
    name: String,
    builder_name: String,
    fields: Vec<Fd>,
}

impl BuilderContext {
    fn new(input: TokenStream) -> Self {
        let (name, fields) = parse(input);
        Self {
            builder_name: format!("{name}Builder"),
            name,
            fields,
        }
    }

    /// 把模板渲染成字符串代码
    pub fn render(input: TokenStream) -> Result<String> {
        let template = Self::new(input);
        Ok(template.render()?)
    }
}

/// 获取结构体名称和所有字段名
fn parse(input: TokenStream) -> (String, Vec<Fd>) {
    let mut input = input.into_iter().collect::<VecDeque<_>>();

    // 解析
    while let Some(item) = input.pop_front() {
        if let TokenTree::Ident(v) = item {
            if "struct" == v.to_string() {
                break;
            }
        }
    }

    let name;
    if let Some(TokenTree::Ident(v)) = input.pop_front() {
        name = v.to_string();
    } else {
        panic!("Didn't find struct name");
    }

    // 找到结构体中的 {...} 部分
    let mut group = None;
    for item in input {
        if let TokenTree::Group(g) = item {
            group = Some(g);
            break;
        }
    }

    // 转换成新的 TokenTree 列表
    let tokens = group.expect("Didn't find field group").stream()
        .into_iter()
        .collect::<Vec<_>>();

    let fds = tokens
        // 按, 分割切片，得到各字段的 TokenTree 切片的迭代器: &[name, ":", type]
        .split(|t| match t {
            TokenTree::Punct(p) => p.as_char() == ',',
            _ => false,
        })
        .map(|field| {
            field
                // 按: 分割切片，返回字段名和字段类型的 TokenTree 切片的迭代器: &[name] &[type]
                .split(|f| match f {
                    TokenTree::Punct(p) => p.as_char() == ':',
                    _ => false,
                })
                .collect::<Vec<_>>()
        })
        .filter(|f| f.len() == 2)
        .map(|f| Fd::new(f[0], f[1]))
        .collect();

    (name, fds)
}
