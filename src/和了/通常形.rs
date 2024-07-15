use crate::牌::牌;

#[must_use]
/// # Panics
/// 手牌の長さが 3n+2 枚でない場合
pub fn is通常形(手牌: &[牌]) -> bool {
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
    手牌: &[牌], index: usize
) -> Vec<Owned手牌> {
    todo!()
}

/// 手牌: 133459中中, index: 1 なら、1{3}3459中中 の 3 およびその右から刻子候補を抜き出す。
/// - 3中中 を抜いた「1 3459」
/// - 33中 を抜いた「1 459中」
/// の 2 通りを返す。
#[must_use]
pub fn 自身より右から刻子を取り除く(
    手牌: &[牌], index: usize
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

    // 自身の直後に同一牌があることが確認できた
    // 自身の 2 枚後に同一牌があるかどうかを見る
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

    // 自身の 2 枚後に同一牌があることが確認できた

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
    assert_eq!(format!("{:?}", result[0]), "[1索#0, 3索#1, 4索#1, 5索#1, 9索#1]");
    assert_eq!(format!("{:?}", result[1]), "[1索#0, 4索#1, 5索#1, 9索#1, 中#1]");
}
