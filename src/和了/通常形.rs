use crate::和了::面子除去::自身より右から面子を取り除く;

use super::Sorted手牌;

#[must_use]
/// # Panics
/// 手牌の長さが 3n+2 枚でない場合
pub fn is通常形(手牌: &Sorted手牌) -> bool {
    println!("{}{:?}", "=".repeat(14 - 手牌.len()), 手牌);
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



#[test]
fn test_通常形_九蓮1_true() {
    use crate::牌::牌;
    unsafe {
        assert!(is通常形(&[
            /* 一萬#0 */ 牌::from_id(0b00_0001_00),
            /* 一萬#1 */ 牌::from_id(0b00_0001_01),
            /* 一萬#2 */ 牌::from_id(0b00_0001_10),
            /* 一萬#3 */ 牌::from_id(0b00_0001_11),
            /* 二萬#0 */ 牌::from_id(0b00_0010_00),
            /* 三萬#0 */ 牌::from_id(0b00_0011_00),
            /* 四萬#0 */ 牌::from_id(0b00_0100_00),
            /* 五萬#0 */ 牌::from_id(0b00_0101_00),
            /* 六萬#0 */ 牌::from_id(0b00_0110_00),
            /* 七萬#0 */ 牌::from_id(0b00_0111_00),
            /* 八萬#0 */ 牌::from_id(0b00_1000_00),
            /* 九萬#0 */ 牌::from_id(0b00_1001_00),
            /* 九萬#1 */ 牌::from_id(0b00_1001_01),
            /* 九萬#2 */ 牌::from_id(0b00_1001_10),
        ]));
    }
}


#[test]
fn test_通常形_九蓮2_true() {
    use crate::牌::牌;
    unsafe {
        assert!(is通常形(&[
            /* 一萬#0 */ 牌::from_id(0b00_0001_00),
            /* 一萬#1 */ 牌::from_id(0b00_0001_01),
            /* 一萬#2 */ 牌::from_id(0b00_0001_10),
            /* 二萬#0 */ 牌::from_id(0b00_0010_00),
            /* 二萬#1 */ 牌::from_id(0b00_0010_01),
            /* 三萬#0 */ 牌::from_id(0b00_0011_00),
            /* 四萬#0 */ 牌::from_id(0b00_0100_00),
            /* 五萬#0 */ 牌::from_id(0b00_0101_00),
            /* 六萬#0 */ 牌::from_id(0b00_0110_00),
            /* 七萬#0 */ 牌::from_id(0b00_0111_00),
            /* 八萬#0 */ 牌::from_id(0b00_1000_00),
            /* 九萬#0 */ 牌::from_id(0b00_1001_00),
            /* 九萬#1 */ 牌::from_id(0b00_1001_01),
            /* 九萬#2 */ 牌::from_id(0b00_1001_10),
        ]));
    }
}

#[test]
fn test_通常形_九蓮3_true() {
    use crate::牌::牌;
    unsafe {
        assert!(is通常形(&[
            /* 一萬#0 */ 牌::from_id(0b00_0001_00),
            /* 一萬#1 */ 牌::from_id(0b00_0001_01),
            /* 一萬#2 */ 牌::from_id(0b00_0001_10),
            /* 二萬#0 */ 牌::from_id(0b00_0010_00),
            /* 三萬#0 */ 牌::from_id(0b00_0011_00),
            /* 三萬#1 */ 牌::from_id(0b00_0011_01),
            /* 四萬#0 */ 牌::from_id(0b00_0100_00),
            /* 五萬#0 */ 牌::from_id(0b00_0101_00),
            /* 六萬#0 */ 牌::from_id(0b00_0110_00),
            /* 七萬#0 */ 牌::from_id(0b00_0111_00),
            /* 八萬#0 */ 牌::from_id(0b00_1000_00),
            /* 九萬#0 */ 牌::from_id(0b00_1001_00),
            /* 九萬#1 */ 牌::from_id(0b00_1001_01),
            /* 九萬#2 */ 牌::from_id(0b00_1001_10),
        ]));
    }
}


#[test]
fn test_通常形_九蓮4_true() {
    use crate::牌::牌;
    unsafe {
        assert!(is通常形(&[
            /* 一萬#0 */ 牌::from_id(0b00_0001_00),
            /* 一萬#1 */ 牌::from_id(0b00_0001_01),
            /* 一萬#2 */ 牌::from_id(0b00_0001_10),
            /* 二萬#0 */ 牌::from_id(0b00_0010_00),
            /* 三萬#0 */ 牌::from_id(0b00_0011_00),
            /* 四萬#0 */ 牌::from_id(0b00_0100_00),
            /* 四萬#1 */ 牌::from_id(0b00_0100_01),
            /* 五萬#0 */ 牌::from_id(0b00_0101_00),
            /* 六萬#0 */ 牌::from_id(0b00_0110_00),
            /* 七萬#0 */ 牌::from_id(0b00_0111_00),
            /* 八萬#0 */ 牌::from_id(0b00_1000_00),
            /* 九萬#0 */ 牌::from_id(0b00_1001_00),
            /* 九萬#1 */ 牌::from_id(0b00_1001_01),
            /* 九萬#2 */ 牌::from_id(0b00_1001_10),
        ]));
    }
}


#[test]
fn test_通常形_九蓮5_true() {
    use crate::牌::牌;
    unsafe {
        assert!(is通常形(&[
            /* 一萬#0 */ 牌::from_id(0b00_0001_00),
            /* 一萬#1 */ 牌::from_id(0b00_0001_01),
            /* 一萬#2 */ 牌::from_id(0b00_0001_10),
            /* 二萬#0 */ 牌::from_id(0b00_0010_00),
            /* 三萬#0 */ 牌::from_id(0b00_0011_00),
            /* 四萬#0 */ 牌::from_id(0b00_0100_00),
            /* 五萬#0 */ 牌::from_id(0b00_0101_00),
            /* 五萬#1 */ 牌::from_id(0b00_0101_01),
            /* 六萬#0 */ 牌::from_id(0b00_0110_00),
            /* 七萬#0 */ 牌::from_id(0b00_0111_00),
            /* 八萬#0 */ 牌::from_id(0b00_1000_00),
            /* 九萬#0 */ 牌::from_id(0b00_1001_00),
            /* 九萬#1 */ 牌::from_id(0b00_1001_01),
            /* 九萬#2 */ 牌::from_id(0b00_1001_10),
        ]));
    }
}


#[test]
fn test_通常形_九蓮6_true() {
    use crate::牌::牌;
    unsafe {
        assert!(is通常形(&[
            /* 一萬#0 */ 牌::from_id(0b00_0001_00),
            /* 一萬#1 */ 牌::from_id(0b00_0001_01),
            /* 一萬#2 */ 牌::from_id(0b00_0001_10),
            /* 二萬#0 */ 牌::from_id(0b00_0010_00),
            /* 三萬#0 */ 牌::from_id(0b00_0011_00),
            /* 四萬#0 */ 牌::from_id(0b00_0100_00),
            /* 五萬#0 */ 牌::from_id(0b00_0101_00),
            /* 六萬#0 */ 牌::from_id(0b00_0110_00),
            /* 六萬#1 */ 牌::from_id(0b00_0110_01),
            /* 七萬#0 */ 牌::from_id(0b00_0111_00),
            /* 八萬#0 */ 牌::from_id(0b00_1000_00),
            /* 九萬#0 */ 牌::from_id(0b00_1001_00),
            /* 九萬#1 */ 牌::from_id(0b00_1001_01),
            /* 九萬#2 */ 牌::from_id(0b00_1001_10),
        ]));
    }
}


#[test]
fn test_通常形_九蓮7_true() {
    use crate::牌::牌;
    unsafe {
        assert!(is通常形(&[
            /* 一萬#0 */ 牌::from_id(0b00_0001_00),
            /* 一萬#1 */ 牌::from_id(0b00_0001_01),
            /* 一萬#2 */ 牌::from_id(0b00_0001_10),
            /* 二萬#0 */ 牌::from_id(0b00_0010_00),
            /* 三萬#0 */ 牌::from_id(0b00_0011_00),
            /* 四萬#0 */ 牌::from_id(0b00_0100_00),
            /* 五萬#0 */ 牌::from_id(0b00_0101_00),
            /* 六萬#0 */ 牌::from_id(0b00_0110_00),
            /* 七萬#0 */ 牌::from_id(0b00_0111_00),
            /* 七萬#1 */ 牌::from_id(0b00_0111_01),
            /* 八萬#0 */ 牌::from_id(0b00_1000_00),
            /* 九萬#0 */ 牌::from_id(0b00_1001_00),
            /* 九萬#1 */ 牌::from_id(0b00_1001_01),
            /* 九萬#2 */ 牌::from_id(0b00_1001_10),
        ]));
    }
}


#[test]
fn test_通常形_九蓮8_true() {
    use crate::牌::牌;
    unsafe {
        assert!(is通常形(&[
            /* 一萬#0 */ 牌::from_id(0b00_0001_00),
            /* 一萬#1 */ 牌::from_id(0b00_0001_01),
            /* 一萬#2 */ 牌::from_id(0b00_0001_10),
            /* 二萬#0 */ 牌::from_id(0b00_0010_00),
            /* 三萬#0 */ 牌::from_id(0b00_0011_00),
            /* 四萬#0 */ 牌::from_id(0b00_0100_00),
            /* 五萬#0 */ 牌::from_id(0b00_0101_00),
            /* 六萬#0 */ 牌::from_id(0b00_0110_00),
            /* 七萬#0 */ 牌::from_id(0b00_0111_00),
            /* 八萬#0 */ 牌::from_id(0b00_1000_00),
            /* 八萬#1 */ 牌::from_id(0b00_1000_01),
            /* 九萬#0 */ 牌::from_id(0b00_1001_00),
            /* 九萬#1 */ 牌::from_id(0b00_1001_01),
            /* 九萬#2 */ 牌::from_id(0b00_1001_10),
        ]));
    }
}


#[test]
fn test_通常形_九蓮9_true() {
    use crate::牌::牌;
    unsafe {
        assert!(is通常形(&[
            /* 一萬#0 */ 牌::from_id(0b00_0001_00),
            /* 一萬#1 */ 牌::from_id(0b00_0001_01),
            /* 一萬#2 */ 牌::from_id(0b00_0001_10),
            /* 二萬#0 */ 牌::from_id(0b00_0010_00),
            /* 三萬#0 */ 牌::from_id(0b00_0011_00),
            /* 四萬#0 */ 牌::from_id(0b00_0100_00),
            /* 五萬#0 */ 牌::from_id(0b00_0101_00),
            /* 六萬#0 */ 牌::from_id(0b00_0110_00),
            /* 七萬#0 */ 牌::from_id(0b00_0111_00),
            /* 八萬#0 */ 牌::from_id(0b00_1000_00),
            /* 九萬#0 */ 牌::from_id(0b00_1001_00),
            /* 九萬#1 */ 牌::from_id(0b00_1001_01),
            /* 九萬#2 */ 牌::from_id(0b00_1001_10),
            /* 九萬#3 */ 牌::from_id(0b00_1001_11),
        ]));
    }
}
