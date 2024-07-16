use crate::和了::面子除去::自身より右から面子を取り除く;

use super::Sorted手牌;

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

    for i in 0..手牌.len() {
        /* 一面子を取り除く */
        let result = 自身より右から面子を取り除く(手牌, i);

        /* その結果出てきた全ての候補について、「通常形か？」と問う */
        for candidate in result {
            if is通常形(&candidate) {
                return true;
            }
        }
    }

    /* どのように取り除いても通常形にならない */
    false
}
