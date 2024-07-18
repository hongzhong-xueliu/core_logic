# core_logic
core logic as described in https://ouc.repo.nii.ac.jp/record/589/files/Aono.19.PDF and https://baike.baidu.com/item/血流成河麻将/1693459

## How to use

`cargo run` すると、役・ロン・ポン・カン無しの「全部俺」モードが遊べる

牌交換終了後、 ws://localhost:1313 につなぐことで、以下のようなJSONが得られる。

yes 0 | cargo run とすると、便利である。

```javascript
// 最初にただ一度だけ欠色が通知される
< {"Game":{"欠色":["萬","萬","萬","萬"]}}

// 現在表示すべき内容が送られてくる。
< {"Display":{"player_id":0,"手牌":["1萬","2萬","3萬","2筒","3筒","4筒","5筒","6筒","7筒","3索","4索","5索","9索"],"河":[[],[],[],[]],"ツモ牌":"9筒","ツモ和了可能性":false}}


// ツモ和了可能性がtrueの時、クライアントは以下のようなものを投げる必要がある。
// （これの動作検証は現状できていない）
> {"ToWinOnSelfDraw": {"value": true}}


// ここで選択制が帰ってきていれば、クライアントは切る牌を送る
< {"DiscardRequest":"選択制"}

// このように
> {"Discard": {"index": 0}}

// そうでないケースとして、以下のようなものが有る。
// 和了ったあとはツモ切りしかできない為。
// これには返信不要。
< {"DiscardRequest":"ツモ"}

// 無効なインデックスを投げると、無視される。


// 時折このようなメッセージが来る。
// これはゲームが続行不能であることを示す。
< {"Error": {"reason": "hogehoge"}}


```


