use std::io;

fn main() {
    println!("電卓");

    loop {
        // 数字と演算子の入力
        println!("数式を入力してください (例: 3 + 4)。終了するには 'q' を入力します。");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("入力エラー");
        let input = input.trim();

        if input.to_lowercase() == "q" {
            println!("電卓を終了します。");
            break;
        }

        // 数式のパース
        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.len() != 3 {
            println!("無効な入力です。正しい形式で入力してください。");
            continue;
        }

        let operand1: f64 = match tokens[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("無効な数値です: {}", tokens[0]);
                continue;
            }
        };

        let operator = tokens[1];
        let operand2: f64 = match tokens[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("無効な数値です: {}", tokens[2]);
                continue;
            }
        };

        // 演算の実行
        let result = match operator {
            "+" => operand1 + operand2,
            "-" => operand1 - operand2,
            "*" => operand1 * operand2,
            "/" => {
                if operand2 == 0.0 {
                    println!("ゼロで割ることはできません。");
                    continue;
                }
                operand1 / operand2
            },
            _ => {
                println!("無効な演算子です。+、-、*、/ のいずれかを使用してください。");
                continue;
            }
        };

        println!("結果: {}", result);
    }
}
