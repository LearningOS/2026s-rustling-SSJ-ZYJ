// errors2.rs
//
// 假设我们正在编写一个游戏，玩家可以用积分购买物品。所有物品的价格都是
// 5 个积分，并且每次购买物品时都需要支付 1 个积分的处理费。游戏中的玩家
// 会输入他们想要购买的物品数量，而 `total_cost` 函数会计算所需的积分总数。
// 但由于玩家是通过字符串输入数量的，我们得到的实际上是字符串类型——而且
// 他们可能输入了任何内容，而不仅仅是数字！
//
// 目前，这个函数根本没有处理错误情况（也没有正确处理成功情况）。我们想要
// 实现的是：如果我们在一个不是数字的字符串上调用 `parse` 函数，该函数会
// 返回一个 `ParseIntError`，在这种情况下，我们希望立即从函数中返回该错误，
// 而不是继续进行乘法和加法运算。
//
// 实现此功能至少有两种正确的方法——但其中一种要短得多！
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty: i32 = match item_quantity.parse::<i32>() {
        Ok(qty) => qty,
        Err(e) => return Err(e),
    };

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
