use core_logic::牌;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

/// 「全部俺」モード
fn main() {
    // 初期配牌#1
    let mut rng = thread_rng();
    let mut y = 牌::全牌一覧.to_vec();
    y.shuffle(&mut rng);

    let mut 手牌s = [
        y[0..13].to_vec(),
        y[13..26].to_vec(),
        y[26..39].to_vec(),
        y[39..52].to_vec(),
    ];
    let remaining = y[52..].to_vec();
    手牌s[0].sort();
    手牌s[1].sort();
    手牌s[2].sort();
    手牌s[3].sort();

    println!("##########################################################");
    println!("###  あなたは「全部俺」モードでプレイしています。");
    println!("##########################################################");

    println!("\n\n--------------------------------------------------");
    println!("|  手牌交換");
    println!("--------------------------------------------------");

    let mut 交換に出される牌s = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    for (i, 手牌) in 手牌s.iter_mut().enumerate() {
        println!("\n\n========= ↓ 以下、プレイヤー{i}の選択 ↓ =========================");
        loop {
            print!("プレイヤー{i}の手牌: [");
            for (j, 牌) in 手牌.iter().enumerate() {
                if j != 0 {
                    print!(", ");
                }
                print!("{j}: {牌:?}");
            }
            println!("]");
            println!("交換に出す牌: {:?}", 交換に出される牌s[i]);
            println!(
                "交換する牌の index を指定してください。 #{}",
                交換に出される牌s[i].len() + 1
            );
            let index = loop {
                let mut index = String::new();
                std::io::stdin().read_line(&mut index).unwrap();
                let index: usize = index.trim().parse().unwrap();
                if index >= 手牌.len() {
                    println!("index out of bounds");
                    continue;
                }
                break index;
            };
            let 交換する牌 = 手牌.remove(index);
            交換に出される牌s[i].push(交換する牌);

            if 交換に出される牌s[i].len() == 3 {
                break;
            }
        }
    }

    let 交換先offset = rng.gen_range(1..=3);

    println!("\n\n--------------------------------------------------");
    println!(
        "|  この卓では、3 枚が{}へ送られます。",
        ["", "下家", "対面", "上家"][交換先offset]
    );
    println!("--------------------------------------------------");

    for (i, 送る3牌) in 交換に出される牌s.iter().enumerate() {
        let 送る先 = (i + 交換先offset) % 4;
        for 送る牌 in 送る3牌 {
            手牌s[送る先].push(*送る牌);
        }
        手牌s[送る先].sort();
    }

    println!("\n\n--------------------------------------------------");
    println!("|  欠色選択");
    println!("--------------------------------------------------");

    let mut 欠色s: [u8; 4] = [0; 4];
    for (i, 欠色) in &mut 欠色s.iter_mut().enumerate() {
        println!("\n\n========= ↓ 以下、プレイヤー{i}の選択 ↓ =========================");
        *欠色 = loop {
            println!("プレイヤー{i}の手牌: {:?}", 手牌s[i]);
            println!("プレイヤー{i}の欠色を入力してください。(0: 萬, 1: 筒, 2: 索)");
            let mut 欠色 = String::new();
            std::io::stdin().read_line(&mut 欠色).unwrap();
            let 欠色: u8 = 欠色.trim().parse().unwrap();
            if 欠色 > 2 {
                println!("0, 1, 2 のいずれかを入力してください。");
                continue;
            }
            break 欠色;
        };
    }
    println!("プレイヤー0の欠色: {}", 欠色s[0]);
    println!("プレイヤー1の欠色: {}", 欠色s[1]);
    println!("プレイヤー2の欠色: {}", 欠色s[2]);
    println!("プレイヤー3の欠色: {}", 欠色s[3]);

    // TODO: 手番を順番に回し、ツモ牌を教え、ツモ上がり可能であるかを判定し、上がらないならクライアントから捨てるべき牌の情報を受け取る
}
