// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.
// 执行 `rustlings hint options3` 或使用 `hint` watch 子命令获取提示。

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(Point { ref x, ref y }) => println!("Co-ordinates are {},{} ", x, y),
        _ => panic!("no match!"),
    }
    // 修复这个错误，但不要删除这行。
    y; // Fix without deleting this line.
}
