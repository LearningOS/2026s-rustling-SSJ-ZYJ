// errors3.rs
//
// 这是一个尝试使用上一题中已完成版本的 `total_cost` 函数的程序。
// 但它现在无法正常工作！为什么？我们应该如何修复它？
//
// 执行 `rustlings hint errors3` 或使用 `hint` watch 子命令获取提示。

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
