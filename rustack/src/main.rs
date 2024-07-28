fn main() {
    // 標準入力の結果を行ごとに取得し、空白区切りで取得
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            // 空白区切り
            let words: Vec<_> = line.split(" ").collect();
            println!("Line: {words:?}");
        }
    }
}

/// 加算演算
/// 
/// # Examples
/// 
/// ```
/// let mut stack = vec![];
/// stack.push(1);
/// stack.push(2);
/// 
/// assert_eq!(6, add(3));
///  ```
fn add(stack: &mut Vec<i32>) {
    // unwrapは値がOKを前提に取り出すため、
    // NoneやErrの場合panic!マクロが呼ばれてクラッシュする
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();
    stack.push(lhs + rhs);
}
