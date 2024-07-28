fn main() {
    let mut stack = vec![];

    stack.push(42);
    stack.push(36);

    add(&mut stack);

    stack.push(22);

    add(&mut stack);

    println!("stack: {stack:?}");
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
