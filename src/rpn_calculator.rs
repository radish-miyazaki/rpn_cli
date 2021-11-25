use anyhow::{bail, ensure, Context, Result};

pub struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    // 計算結果を出力
    pub fn eval(&self, formula: &str) -> Result<i32> {
        // 数式文字列をスペースで区切って、逆順にし、collect型に変換する
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    // 実際の計算処理を行う関数
    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                // 数値に変換できる場合は、stackにPush
                stack.push(x);
            } else {
                // 数値に変換できない場合は、演算子と判断し、スタックから2つの値を取得
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;

                // パターンマッチで処理を分ける
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    // 上記以外の演算子が入ってきた場合はpanic!
                    _ => bail!("invalid token at {}", pos),
                };
                // 結果をVecに格納
                stack.push(res);
            }

            // -v オプションがセットされている場合は、この時点でのトークンとスタックの状態を出力
            if self.0 {
                println!("{:?} {:?}", tokens, stack)
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        // 数値だけが入力された場合の処理
        assert_eq!(calc.eval("5").ok(), Some(5));
        assert_eq!(calc.eval("50").ok(), Some(50));
        assert_eq!(calc.eval("-50").ok(), Some(-50));

        // 2個の数値と演算子1つの場合
        assert_eq!(calc.eval("2 3 +").ok(), Some(5));
        assert_eq!(calc.eval("2 3 *").ok(), Some(6));
        assert_eq!(calc.eval("2 3 -").ok(), Some(-1));
        assert_eq!(calc.eval("2 3 /").ok(), Some(0));
        assert_eq!(calc.eval("2 3 %").ok(), Some(2));
    }

    #[test]
    fn test_ng() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("1 1 ^").ok(), None);
    }
}