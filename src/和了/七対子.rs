use itertools::Itertools;

use crate::牌::牌の種類;

use super::Sorted手牌;

// 四枚使い・ワイルドカードを許す
// ソート済みを前提とする
#[must_use]
pub fn is七対子(手牌: &Sorted手牌) -> bool {
    if 手牌.len() != 14 {
        return false;
    }

    // 「すべての種類について、ワイルドカードを上手く使うことで偶数枚にできるか？」という問い
    // つまり、「奇数枚ある種類」の個数が、ワイルドカードの枚数以下であるか？　を問えばよい

    let mut 奇数種類の個数 = 0;
    let mut ワイルドカードの枚数 = 0;

    for ((種類, _), chunk) in &手牌.iter().chunk_by(|elt| (elt.種類(), elt.数())) {
        if 種類 == 牌の種類::中 as u8 {
            // 中はワイルドカードとして使える
            ワイルドカードの枚数 = chunk.count();
        } else if chunk.count() % 2 == 1 {
            奇数種類の個数 += 1;
        }
    }

    奇数種類の個数 <= ワイルドカードの枚数
}

#[test]
fn test_七対子_四枚使い_true() {
    use crate::牌::牌;
    unsafe {
        assert!(is七対子(&[
            /* 一萬#0 */ 牌::from_id(0b00_0001_00),
            /* 一萬#1 */ 牌::from_id(0b00_0001_01),
            /* 二萬#0 */ 牌::from_id(0b00_0010_00),
            /* 二萬#1 */ 牌::from_id(0b00_0010_01),
            /* 一筒#0 */ 牌::from_id(0b01_0001_00),
            /* 一筒#1 */ 牌::from_id(0b01_0001_01),
            /* 二筒#0 */ 牌::from_id(0b01_0010_00),
            /* 二筒#1 */ 牌::from_id(0b01_0010_01),
            /* 六筒#2 */ 牌::from_id(0b01_0110_10),
            /* 六筒#3 */ 牌::from_id(0b01_0110_11),
            /* 七索#0 */ 牌::from_id(0b10_0111_00),
            /* 七索#1 */ 牌::from_id(0b10_0111_01),
            /* 七索#2 */ 牌::from_id(0b10_0111_10),
            /* 七索#3 */ 牌::from_id(0b10_0111_11),
        ]));
    }
}

#[test]
fn test_七対子_四枚使い_false() {
    use crate::牌::牌;
    use std::ops::Not;
    unsafe {
        assert!(is七対子(&[
            /* 一萬#0 */ 牌::from_id(0b00_0001_00),
            /* 一萬#1 */ 牌::from_id(0b00_0001_01),
            /* 二萬#0 */ 牌::from_id(0b00_0010_00),
            /* 五萬#1 */ 牌::from_id(0b00_0101_01),
            /* 一筒#0 */ 牌::from_id(0b01_0001_00),
            /* 一筒#1 */ 牌::from_id(0b01_0001_01),
            /* 二筒#0 */ 牌::from_id(0b01_0010_00),
            /* 二筒#1 */ 牌::from_id(0b01_0010_01),
            /* 六筒#2 */ 牌::from_id(0b01_0110_10),
            /* 六筒#3 */ 牌::from_id(0b01_0110_11),
            /* 七索#0 */ 牌::from_id(0b10_0111_00),
            /* 七索#1 */ 牌::from_id(0b10_0111_01),
            /* 七索#2 */ 牌::from_id(0b10_0111_10),
            /* 七索#3 */ 牌::from_id(0b10_0111_11),
        ])
        .not());
    }
}

#[test]
fn test_七対子_四枚使いとワイルドカード_true() {
    use crate::牌::牌;
    unsafe {
        assert!(is七対子(&[
            /* 一萬#0 */ 牌::from_id(0b00_0001_00),
            /* 一萬#1 */ 牌::from_id(0b00_0001_01),
            /* 二萬#0 */ 牌::from_id(0b00_0010_00),
            /* 二萬#1 */ 牌::from_id(0b00_0010_01),
            /* 一筒#0 */ 牌::from_id(0b01_0001_00),
            /* 一筒#1 */ 牌::from_id(0b01_0001_01),
            /* 二筒#0 */ 牌::from_id(0b01_0010_00),
            /* 二筒#1 */ 牌::from_id(0b01_0010_01),
            /* 六筒#3 */ 牌::from_id(0b01_0110_11),
            /* 七索#0 */ 牌::from_id(0b10_0111_00),
            /* 七索#1 */ 牌::from_id(0b10_0111_01),
            /* 七索#2 */ 牌::from_id(0b10_0111_10),
            /* 七索#3 */ 牌::from_id(0b10_0111_11),
            /*   中#1 */ 牌::from_id(0b11_1111_01),
        ]));
    }
}

#[test]
fn test_七対子_四枚使いとワイルドカード_false() {
    use crate::牌::牌;
    use std::ops::Not;
    unsafe {
        assert!(is七対子(&[
            /* 一萬#0 */ 牌::from_id(0b00_0001_00),
            /* 一萬#1 */ 牌::from_id(0b00_0001_01),
            /* 二萬#0 */ 牌::from_id(0b00_0010_00),
            /* 二萬#1 */ 牌::from_id(0b00_0010_01),
            /* 一筒#0 */ 牌::from_id(0b01_0001_00),
            /* 一筒#1 */ 牌::from_id(0b01_0001_01),
            /* 二筒#0 */ 牌::from_id(0b01_0010_00),
            /* 三筒#1 */ 牌::from_id(0b01_0011_01),
            /* 六筒#3 */ 牌::from_id(0b01_0110_11),
            /* 七索#0 */ 牌::from_id(0b10_0111_00),
            /* 七索#1 */ 牌::from_id(0b10_0111_01),
            /* 七索#2 */ 牌::from_id(0b10_0111_10),
            /* 七索#3 */ 牌::from_id(0b10_0111_11),
            /*   中#1 */ 牌::from_id(0b11_1111_01),
        ])
        .not());
    }
}
