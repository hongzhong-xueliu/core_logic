use crate::牌::牌;

// TODO: ソートされているという前提を課しているので、ソートされていないものが誤って入ってこないように private struct でも作って守る
type Sorted手牌 = [牌];

#[must_use]
/// # Panics
/// 手牌の長さが 3n+2 枚でない場合
pub fn is通常形(手牌: &Sorted手牌) -> bool {
    assert!(
        手牌.len() % 3 == 2,
        "手牌の長さが不正です。この関数には 3n+2 枚の手牌が必要です。"
    );

    if 手牌.len() == 2 {
        /* ソートされているという前提なので、
        「2 番目がワイルドカードである」または「両方が同一牌である」で充分 */
        return 手牌[1].isワイルドカード() || 手牌[0].is同一牌(手牌[1]);
    }

    todo!()
}

type Owned手牌 = Vec<牌>;

/// 手牌: 133459中中, index: 2 なら、13{3}459中中 の {3} およびその右から面子候補を抜き出す。
/// - 3中中 を抜いた「13 459」
/// - 34中 を抜いた「13 59中」
/// - 3中5 を抜いた「13 49中」
/// - 345 を抜いた「13 9中中」
/// の 4 通りを返す。
#[must_use]
pub fn 自身より右から面子を取り除く(
    手牌: &Sorted手牌,
    index: usize,
) -> Vec<Owned手牌> {
    [
        自身より右から順子を取り除く(手牌, index),
        自身より右から刻子を取り除く(手牌, index),
    ]
    .concat()
}

fn 次牌のindex(手牌: &Sorted手牌) -> Option<usize> {
    let 自身 = 手牌[0];
    (1..手牌.len()).find(|&i| 手牌[i].is次牌of(自身))
}

/// 「中2枚を含む」および「中3枚」は刻子としてみなす。
/// 手牌: 133459中中, index: 1 なら、1{3}3459中中 の 3 およびその右から順子候補を抜き出すので、
/// - 34中 を抜いた「1 359中」
/// - 3中5 を抜いた「1 349中」
/// - 345 を抜いた「1 39中中」
/// の 3 通りを返す。
#[must_use]
pub fn 自身より右から順子を取り除く(
    手牌: &Sorted手牌,
    index: usize,
) -> Vec<Owned手牌> {
    let left = &手牌[..index];
    let 自身 = &手牌[index];
    let right = &手牌[index + 1..];

    // あと 2 枚を取り除くことができない場合をあらかじめ排除
    if right.len() < 2 {
        return vec![];
    }

    if 自身.isワイルドカード() {
        // 自身より右にはワイルドカードしかない。これは「中3枚」の刻子として扱う
        // よって、順子は作れないので解なし
        return vec![];
    }

    // 真右の牌が「次牌」または「次次牌」であるかどうかを見る

    // 真右が「次次牌」なら、わりと簡単
    if right[0].is次次牌of(*自身) {
        // 真右が「次次牌」なので、ワイルドカードを消費するしかない
        // 「ワイルドカードを 1 枚消費する」のみが許される

        if right[right.len() - 1].isワイルドカード() {
            // ワイルドカードを 1 枚、次次牌を 1 枚消費する
            // a[bcde]f
            let result: Owned手牌 = [left, &right[1..right.len() - 1]].concat();
            return vec![result];
        }

        return vec![];
    }

    // 真右が「次牌」かどうかを見る。右に順子で伸ばせないなら、簡単。
    if !right[0].is次牌of(*自身) {
        // 真右に、右に順子を伸ばすための牌がない
        // ワイルドカードを 2 枚消費するのが唯一解
        // それは順子扱いしないことにしたので、解なし
        return vec![];
    }

    // 真右に来る牌が「次牌」。さらに右を見なくてはいけない
    // ここで、
    // - 123 パターン（おめでとう）
    // - 124 パターン（ワイルド消費義務）
    // - 122 パターン（123 を作れる可能性が残されている）
    // があり、めんどい
    let ふたつ右 = right[1];
    if ふたつ右.is同一牌(right[0]) {
        // 122 パターン。めんどい
        let ワイルドカードがある = right[right.len() - 1].isワイルドカード();
        let 次次牌index: Option<usize> = 次牌のindex(right);
        match (ワイルドカードがある, 次次牌index) {
            (false, None) => {
                // 122 パターンで、ワイルドカードも次次牌もないので解なし
                vec![]
            }
            (true, None) => {
                // 122 パターンで、ワイルドカードがあるが次次牌がないので、
                // 12 と 中 のみ取り除いて返す
                let result: Owned手牌 = [
                    left,
                    /* 自身と right[0] がなくて */
                    &right[1..right.len() - 1], /* 末尾の中がない */
                ]
                .concat();
                vec![result]
            }
            (false, Some(index)) => {
                // 122 パターンで、ワイルドカードがないが次次牌があるので、
                // 123 のみ取り除いて返す
                let result: Owned手牌 = [
                    left,
                    /* 自身と right[0] がなくて */
                    &right[1..index], /* right[index] がない */
                    &right[index + 1..right.len()],
                ]
                .concat();
                vec![result]
            }
            (true, Some(index)) => {
                // 122 パターンで、ワイルドカードも次次牌もあるので、
                // 123, 12中, 1中3 の 3 通りを取り除いたものを返す
                vec![
                    [
                        left,
                        /* 自身と right[0] がなくて */
                        &right[1..index], /* right[index] がない */
                        &right[index + 1..right.len()],
                    ] /* これで 123 の除去 */
                    .concat(),
                    [
                        left,
                        /* 自身と right[0] がなくて */
                        &right[1..right.len() - 1], /* 末尾の中がない */
                    ] /* これで 12中の除去 */
                    .concat(),
                    [
                        left,                               /* 自身がなくて */
                        &right[0..index],                   /* right[index] がなくて */
                        &right[index + 1..right.len() - 1], /* 末尾の中がない */
                    ] /* これで 1中3の除去 */
                    .concat(),
                ]
            }
        }
    } else if ふたつ右.is次牌of(right[0]) {
        // 123 パターン。おめでとう
        // ワイルドカードがないなら、123 のみ取り除いて返す

        let remove_123 = [left, &right[2..]].concat();

        if !right[right.len() - 1].isワイルドカード() {
            return vec![remove_123];
        }

        // ワイルドカードがあるなら、123, 12中, 1中3 の 3 通りを取り除いたものを返す
        return vec![
            remove_123,
            [
                left,
                /* 自身 および right[0] がなくて */
                &right[1..right.len() - 1], /* 末尾の中がない */
            ]
            .concat(),
            [
                left,
                /* 自身がなくて */ &[right[0]],
                /* right[1] がなくて */
                &right[2..right.len() - 1], /* 末尾の中がない */
            ]
            .concat(),
        ];
    } else {
        // 124 パターン。ワイルド消費義務
        // ワイルドは 1 枚のみ消費が許される。「12中」の形でしか伸びない
        // これが唯一解
        if right[right.len() - 1].isワイルドカード() {
            let result: Owned手牌 = [left, &right[1..right.len() - 1]].concat();
            return vec![result];
        }
        return vec![];
    }
}

/// 手牌: 133459中中, index: 1 なら、1{3}3459中中 の 3 およびその右から刻子候補を抜き出す。
/// - 3中中 を抜いた「1 3459」
/// - 33中 を抜いた「1 459中」
/// の 2 通りを返す。
#[must_use]
pub fn 自身より右から刻子を取り除く(
    手牌: &Sorted手牌,
    index: usize,
) -> Vec<Owned手牌> {
    let left = &手牌[..index];
    let 自身 = &手牌[index];
    let right = &手牌[index + 1..];

    // あと 2 枚を取り除くことができない場合をあらかじめ排除
    if right.len() < 2 {
        return vec![];
    }

    if 自身.isワイルドカード() {
        // 自身より右にはワイルドカードしかないので、そいつらのうち 2 枚を取り除くだけでよい
        // これが唯一解
        let result: Owned手牌 = [left, &right[2..]].concat();
        return vec![result];
    }

    if !自身.is同一牌(right[0]) {
        // 刻子を作るためには、自身と同じ牌が 2 枚以上必要
        // ワイルドカードを 2 枚消費するのが唯一解
        // ソートされている前提なので、ワイルドカードが「最後から 2 枚目」にあるかどうかが判定条件
        if right[right.len() - 2].isワイルドカード() {
            let result: Owned手牌 = [left, &right[..right.len() - 2]].concat();
            return vec![result];
        }

        // さもなければ、ワイルドカードが足りないので解なし
        return vec![];
    }

    // 自身の真右に同一牌があることが確認できた
    // 自身の 2 枚右に同一牌があるかどうかを見る
    if !自身.is同一牌(right[1]) {
        // 自身が足りない
        // 「ワイルドカードを 1 枚消費する」または「ワイルドカードを 2 枚消費する」が許される

        let mut candidate: Vec<Owned手牌> = vec![];

        // 「ワイルドカードを 2 枚消費する」のとき
        // [abcd]ef
        if right[right.len() - 2].isワイルドカード() {
            let result: Owned手牌 = [left, &right[..right.len() - 2]].concat();
            candidate.push(result);
        }

        // ワイルドカードを 1 枚、自身を 1 枚消費する
        // a[bcde]f
        if right[right.len() - 1].isワイルドカード() {
            let result: Owned手牌 = [left, &right[1..right.len() - 1]].concat();
            candidate.push(result);
        }

        return candidate;
    }

    // 自身の 2 枚右に同一牌があることが確認できた

    // 「ワイルドカードを 0 枚消費する」または「ワイルドカードを 1 枚消費する」または「ワイルドカードを 2 枚消費する」が許される

    let mut candidate: Vec<Owned手牌> = vec![];

    // 「ワイルドカードを 2 枚消費する」のとき
    // [abcd]ef
    if right[right.len() - 2].isワイルドカード() {
        let result: Owned手牌 = [left, &right[..right.len() - 2]].concat();
        candidate.push(result);
    }

    // ワイルドカードを 1 枚、自身を 1 枚消費する
    // a[bcde]f
    if right[right.len() - 1].isワイルドカード() {
        let result: Owned手牌 = [left, &right[1..right.len() - 1]].concat();
        candidate.push(result);
    }

    // 自身を 2 枚消費する
    // ab[cdef]
    if right[right.len() - 1].isワイルドカード() {
        let result: Owned手牌 = [left, &right[2..right.len()]].concat();
        candidate.push(result);
    }

    candidate
}

#[test]
fn 刻子除去() {
    // 手牌: 133459中中, index: 1 なら、1{3}3459中中 の 3 およびその右から刻子候補を抜き出す。
    // - 3中中 を抜いた「1 3459」
    // - 33中 を抜いた「1 459中」

    let 手牌 = unsafe {
        [
            牌::from_id(0b10_0001_00), // 1索#0
            牌::from_id(0b10_0011_00), // 3索#0
            牌::from_id(0b10_0011_01), // 3索#1
            牌::from_id(0b10_0100_01), // 4索#1
            牌::from_id(0b10_0101_01), // 5索#1
            牌::from_id(0b10_1001_01), // 9索#1
            牌::from_id(0b11_1111_01), // 中#1
            牌::from_id(0b11_1111_10), // 中#2
        ]
    };

    let result = 自身より右から刻子を取り除く(&手牌, 1);
    assert_eq!(result.len(), 2);

    // TODO: この確認方法は良くなくて、というのも「どういう順番で取り除くか」に依存している
    // 正しくは、詳細ID を無視して、牌の種類だけで比較するべき
    assert_eq!(
        format!("{:?}", result[0]),
        "[1索#0, 3索#1, 4索#1, 5索#1, 9索#1]"
    );
    assert_eq!(
        format!("{:?}", result[1]),
        "[1索#0, 4索#1, 5索#1, 9索#1, 中#1]"
    );
}
