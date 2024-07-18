use core_logic::牌;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

use std::cell::OnceCell;

#[derive(Debug, Deserialize)]
struct Discard {
    index: usize,
}

#[derive(Debug, Deserialize)]
struct ToWinOnSelfDraw {
    value: bool,
}

#[derive(Debug, Deserialize)]
struct ClientHello;

#[derive(Debug, Deserialize)]
enum ClientMessage {
    ClientHello(ClientHello),
    Discard(Discard),
    ToWinOnSelfDraw(ToWinOnSelfDraw),
}

#[derive(Debug, Serialize)]
struct Game {
    欠色: [String; 4],
}

#[derive(Debug, Serialize)]
struct Display {
    player_id: usize,
    手牌: Vec<String>,
    河: Vec<Vec<String>>,
    ツモ牌: String,
    ツモ和了可能性: bool,
}

#[derive(Debug, Serialize)]
struct Error {
    reason: String,
}

#[derive(Debug, Serialize)]
enum DiscardRequest {
    ツモ,
    選択制,
}

#[derive(Debug, Serialize)]
enum ServerMessage {
    Game(Game),
    Display(Display),
    DiscardRequest(DiscardRequest),
    Error(Error),
}

static SOCKET: Mutex<OnceCell<(Sender<ClientMessage>, Receiver<ServerMessage>)>> =
    Mutex::const_new(OnceCell::new());

async fn handle_socket(mut socket: WebSocket) {
    println!("Connection established!");

    let Some((c_tx, mut s_rx)) = SOCKET.lock().await.take() else {
        println!("Connection is already taken.");
        socket
            .send(Message::Text(
                serde_json::to_string(&ServerMessage::Error(Error {
                    reason: "Connection is already taken".to_string(),
                }))
                .unwrap(),
            ))
            .await
            .unwrap();

        return;
    };

    let (mut sender, mut receiver) = socket.split();

    tokio::spawn(async move {
        loop {
            sender
                .send(Message::Text(
                    serde_json::to_string(&s_rx.recv().await.unwrap()).unwrap(),
                ))
                .await
                .unwrap()
        }
    });

    loop {
        let Some(message) = receiver.next().await else {
            println!("WebSocket Connection has been closed!");
            std::process::exit(1);
        };

        let Message::Text(m) = message.unwrap() else {
            continue;
        };

        c_tx.send(serde_json::from_str(&m).unwrap()).await.unwrap();
    }
}

async fn handler(ws: WebSocketUpgrade) -> axum::response::Response {
    ws.on_upgrade(handle_socket)
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:1313").await.unwrap();

    let (s_tx, s_rx) = tokio::sync::mpsc::channel(32);
    let (c_tx, mut c_rx) = tokio::sync::mpsc::channel(32);

    SOCKET.lock().await.set((c_tx, s_rx)).unwrap();

    tokio::spawn(async move {
        axum::serve(listener, axum::Router::new().fallback(handler))
            .await
            .unwrap();
    });

    let mut game_state = initialize_with_全部俺();

    s_tx.send(ServerMessage::Game(Game {
        欠色: [
            game_state.player_data[0].欠色.to_string(),
            game_state.player_data[1].欠色.to_string(),
            game_state.player_data[2].欠色.to_string(),
            game_state.player_data[3].欠色.to_string(),
        ],
    }))
    .await
    .unwrap();

    println!("\n\n--------------------------------------------------");
    println!("|  打牌開始");
    println!("--------------------------------------------------");

    let mut current_player = 0;

    loop {
        let Some(ツモ牌) = game_state.remaining.pop() else {
            println!("山が空になりました。ゲーム終了です。");

            s_tx.send(ServerMessage::Error(Error {
                reason: "山が空".to_string(),
            }))
            .await
            .unwrap();

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            break;
        };

        println!(
            "\n\n========= ↓ 以下、プレイヤー{current_player}の選択 ↓ ========================="
        );

        let 手牌 = game_state.player_data[current_player].手牌.clone();
        println!("あなたの手牌: {手牌:?}",);

        let 河 = game_state
            .player_data
            .iter()
            .map(|p| p.河.iter().map(|牌| 牌.to_string()).collect())
            .collect::<Vec<_>>();

        println!("河: {河:?}",);
        println!("ツモ牌: {ツモ牌:?}");

        let ツモ和了可能性 = core_logic::和了::待ち牌(
            &game_state.player_data[current_player].手牌,
            game_state.player_data[current_player].欠色,
        )
        .contains(&ツモ牌);

        s_tx.send(ServerMessage::Display(Display {
            player_id: current_player,
            手牌: 手牌.iter().map(|牌| 牌.to_string()).collect(),
            河,
            ツモ牌: ツモ牌.to_string(),
            ツモ和了可能性,
        }))
        .await
        .unwrap();

        if ツモ和了可能性 {
            println!("ツモ和了りできます。和了りますか？ (Y/n)");

            let 和了る = loop {
                if let ClientMessage::ToWinOnSelfDraw(ToWinOnSelfDraw { value }) =
                    c_rx.recv().await.unwrap()
                {
                    break value;
                }
            };

            if 和了る {
                println!("和了りませんでした。");
            } else {
                println!("和了りました。");
                game_state.player_data[current_player].和了回数 += 1;
            }
        }

        // 既に和了済の場合は、ツモ切りしかできない
        if game_state.player_data[current_player].和了回数 > 0 {
            println!("ツモ切ります。");
            s_tx.send(ServerMessage::DiscardRequest(DiscardRequest::ツモ))
                .await
                .unwrap();

            game_state.player_data[current_player].河.push(ツモ牌);
        } else {
            // そうでない場合は、ツモ切りか捨てる牌を選択する
            s_tx.send(ServerMessage::DiscardRequest(DiscardRequest::選択制))
                .await
                .unwrap();

            print!("あなたの手牌: [");
            for (j, 牌) in game_state.player_data[current_player]
                .手牌
                .iter()
                .enumerate()
            {
                if j != 0 {
                    print!(", ");
                }
                print!("{j}: {牌:?}");
            }
            println!(
                "] + [{}: {ツモ牌:?}]",
                game_state.player_data[current_player].手牌.len()
            );

            println!("捨てる牌の index を指定してください。",);
            let index = loop {
                if let ClientMessage::Discard(Discard { index }) = c_rx.recv().await.unwrap() {
                    if index > game_state.player_data[current_player].手牌.len() {
                        println!("index out of bounds. try again");
                        continue;
                    }
                    break index;
                }
            };
            game_state.player_data[current_player].手牌.push(ツモ牌);
            let 捨て牌 = game_state.player_data[current_player].手牌.remove(index);
            game_state.player_data[current_player].手牌.sort();
            game_state.player_data[current_player].河.push(捨て牌);
        }

        current_player = (current_player + 1) % 4;
    }
}

struct PlayerData {
    手牌: Vec<牌::牌>,
    欠色: 牌::色,
    和了回数: u32,
    河: Vec<牌::牌>,
}

struct InitializedGameState {
    player_data: [PlayerData; 4],
    remaining: Vec<牌::牌>,
}

fn initialize_with_全部俺() -> InitializedGameState {
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
                    println!("index out of bounds. try again");
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

    let mut 欠色s: [牌::色; 4] = [牌::色::中; 4];
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
            break unsafe { std::mem::transmute(欠色) };
        };
    }
    println!("プレイヤー0の欠色: {}", 欠色s[0]);
    println!("プレイヤー1の欠色: {}", 欠色s[1]);
    println!("プレイヤー2の欠色: {}", 欠色s[2]);
    println!("プレイヤー3の欠色: {}", 欠色s[3]);

    InitializedGameState {
        remaining,
        player_data: [
            PlayerData {
                手牌: 手牌s[0].clone(),
                欠色: 欠色s[0],
                和了回数: 0,
                河: Vec::new(),
            },
            PlayerData {
                手牌: 手牌s[1].clone(),
                欠色: 欠色s[1],
                和了回数: 0,
                河: Vec::new(),
            },
            PlayerData {
                手牌: 手牌s[2].clone(),
                欠色: 欠色s[2],
                和了回数: 0,
                河: Vec::new(),
            },
            PlayerData {
                手牌: 手牌s[3].clone(),
                欠色: 欠色s[3],
                和了回数: 0,
                河: Vec::new(),
            },
        ],
    }
}
