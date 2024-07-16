use crate::牌::牌;

#[path = "七対子.rs"]
pub mod 七対子;

#[path = "通常形.rs"]
pub mod 通常形;

#[path = "面子除去.rs"]
pub mod 面子除去;

// TODO: ソートされているという前提を課しているので、ソートされていないものが誤って入ってこないように private struct でも作って守る
pub type Sorted手牌 = [牌];
