use std::io;

fn main() {
    println!("じゃんけんゲームを開始!");

    loop {
        // ユーザーの選択を取得
        println!("選択肢: 1: グー, 2: チョキ, 3: パー");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("入力エラー");
        let user_choice: u32 = match user_input.trim().parse() {
            Ok(num) if num >= 1 && num <= 3 => num,
            _ => {
                println!("無効な選択です。1、2、または3を選んでください");
                continue;
            },
        };

        // コンピュータの選択をランダムに生成
        let computer_choice = rand::random::<u32>() % 3 + 1;

        // 選択を表示
        println!("あなたの選択: {}", match user_choice {
            1 => "グー",
            2 => "チョキ",
            3 => "パー",
            _ => unreachable!(),
        });
        println!("コンピュータの選択: {}", match computer_choice {
            1 => "グー",
            2 => "チョキ",
            3 => "パー",
            _ => unreachable!(),
        });

        // 勝敗の判定
        let result = match (user_choice, computer_choice) {
            (1, 2) | (2, 3) | (3, 1) => "あなたの勝ち!",
            (2, 1) | (3, 2) | (1, 3) => "あなたの負け!",
            _ => "引き分け!",
        };
        println!("{}", result);

        // ゲームを続けるかどうかの確認
        println!("もう一度遊びますか？ (y/n)");
        let mut play_again = String::new();
        io::stdin().read_line(&mut play_again).expect("入力エラー");
        if play_again.trim().to_lowercase() != "y" {
            break;
        }
    }

    println!("ゲームを終了します。");
}
