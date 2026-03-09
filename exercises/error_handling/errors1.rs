// errors1.rs
//
// 如果你传给它一个空字符串，这个函数拒绝生成要打印在胸牌上的文本。
// 如果它能解释问题出在哪里，而不是有时只返回 `None`，那就更好了。
// 幸运的是，Rust 有一个与 `Result` 类似的结构，可以用来表达错误情况。让我们使用它吧！
//
// 执行 `rustlings hint errors1` 或使用 `hint` watch 子命令获取提示。

pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
