use crate::牌::牌;

#[path = "七対子.rs"]
pub mod 七対子;

#[path = "通常形.rs"]
pub mod 通常形;

#[path = "面子除去.rs"]
pub mod 面子除去;

// TODO: ソートされているという前提を課しているので、ソートされていないものが誤って入ってこないように private struct でも作って守る
pub type Sorted手牌 = [牌];

#[must_use]
pub fn is和了(手牌: &Sorted手牌) -> bool {
    通常形::is通常形(手牌) || 七対子::is七対子(手牌)
}

#[must_use]
pub fn is和了and欠色(手牌: &Sorted手牌, 欠色: u8) -> bool {
    手牌.iter().filter(|&x| x.色() == 欠色).count() == 0 && is和了(手牌)
}
