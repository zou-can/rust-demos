/// ### 声明宏示例
///
/// `#[macro_export]` 注解表明只要导入了定义这个宏的 crate，该宏就应该是可用的。
/// 如果没有该注解，这个宏不能被引入作用域。
///
/// 声明宏通过匹配输入的模式来分别处理不同的逻辑，可通过 `$入参:入参类型` 指定入参的类型，入参的类型的取值如下：
/// - `item`：比如一个函数、结构体、模块等。
/// - `block`：代码块。比如一系列由花括号包裹的表达式和语句。
/// - `stmt`：语句。比如一个赋值语句。
/// - `pat`：模式。expr，表达式。刚才的例子使用过了。
/// - `ty`：类型。比如 Vec。ident，标识符。比如一个变量名。
/// - `path`：路径。比如：`foo`、`::std::mem::replace`、`transmute::<_, int>`。
/// - `meta`：元数据。一般是在 `#[...]` 和 `#![...]` 属性内部的数据。
/// - `tt`：单个的 token 树。
/// - `vis`：可能为空的一个 `Visibility` 修饰符。比如 pub、pub(crate)。
#[macro_export]
macro_rules! my_vec {
    // my_vec![]
    () => {
        Vec::new()
    };

    // my_vec![1, 2, 3, 4]
    // $x:type 用来指定输入的格式
    // ,为分隔符
    // $(...)* 代表可重复多次
    ($($ex:expr),*) => {
        {
            let mut v = Vec::new();
            // 使用 $(...)* 展开匹配到的输入
            $(v.push($ex);)*
            v
        }
    };

    // my_vec![0; 10]
    ($ex:expr; $n:expr) => {
        std::vec::from_elem($ex, $n)
    };

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut v: Vec<i32> = my_vec![];
        assert!(v.is_empty());

        v = my_vec![1, 2, 3, 4];
        assert_eq!(vec![1, 2, 3, 4], v);

        v = my_vec![1; 3];
        assert_eq!(vec![1; 3], v);
    }
}
