// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint options1` 或使用 `hint` watch 子命令获取提示。

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
// 这个函数返回冰箱里还剩多少冰淇淋。
// 如果是晚上10点前，还有5块。到了10点，有人会把它们全部吃完，
// 所以之后就再也没有了 :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // 这里我们使用24小时制，所以晚上10点是22，凌晨12点是0。
    // Option输出应该能优雅地处理time_of_day > 23的情况。
    // TODO: Complete the function body - remember to return an Option!
    // TODO: 完成函数体 - 记得返回一个Option！
    if time_of_day < 22 {
        Some(5)
    } else if time_of_day >= 22 && time_of_day <= 24 {
        Some(0)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        // TODO: 修复这个测试。如何获取 Option 中包含的值？
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Some(5));
    }
}
