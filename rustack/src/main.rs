fn main() {
    // 標準入力の結果を行ごとに取得し、空白区切りで取得
    for line in std::io::stdin().lines() {
        let mut stack = vec![];
        if let Ok(line) = line {
            // 空白区切り
            let words: Vec<_> = line.split(" ").collect();
            
            // 文字列からパースし、数値ならスタックへPush
            // そうでないなら(四則演算記号)match構文へ
            for word in words {
                if let Ok(parsed) = word.parse::<i32>() {
                    stack.push(parsed);
                } else {
                    match word {
                        "+" => add(&mut stack),
                        "-" => sub(&mut stack),
                        "*" => mul(&mut stack),
                        "/" => div(&mut stack),
                        _ => panic!("{word:?} could not be parsed"),
                    }
                }
            }
            println!("Stack: {stack:?}");
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
/// assert_eq!(6, add(&mut stack));
///  ```
fn add(stack: &mut Vec<i32>) {
    // unwrapは値がOKを前提に取り出すため、
    // NoneやErrの場合panic!マクロが呼ばれてクラッシュする
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();
    stack.push(lhs + rhs);
}

/// 減算演算
/// 
/// # Examples
/// 
/// ```
/// let mut stack = vec![];
/// stack.push(3);
/// stack.push(2);
/// 
/// assert_eq!(1, sub(&mut sub));
/// ```
fn sub(stack: &mut Vec<i32>) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();
    stack.push(lhs - rhs);
}

/// 乗算演算
/// 
/// # Examples
/// 
/// ```
/// let mut stack = vec![];
/// stack.push(3);
/// stack.push(2);
/// 
/// assert_eq!(6, sub(&mut mul));
/// ```
fn mul(stack: &mut Vec<i32>) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();
    stack.push(lhs * rhs);
}

/// 除算演算
/// 
/// # Examples
/// 
/// ```
/// let mut stack = vec![];
/// stack.push(6);
/// stack.push(2);
/// 
/// assert_eq!(3, div(&mut sub));
/// ```
fn div(stack: &mut Vec<i32>) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();
    if rhs == 0 {
        panic!("Divided by zero.");
    }
    stack.push(lhs / rhs);
}
