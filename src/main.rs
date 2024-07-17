use core_logic::牌;
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;

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

    // TODO: 手牌交換

    // 欠色選択
    let mut 欠色s: [u8; 4] = [0; 4];
    for (i, 欠色) in &mut 欠色s.iter_mut().enumerate() {
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
