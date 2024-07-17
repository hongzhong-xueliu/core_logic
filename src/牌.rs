use std::fmt::Debug;
use std::num::NonZeroU8;
/*
 * 上位 2 ビットを種類 (萬 00, 筒 01, 索 10, 中 11)
 * 中央 4 ビットを数 (0001 - 1001)
 * 下位 2 ビットを同種牌内での個別の ID (00 - 11)
 * とすると、u8 でソートするだけで牌がソートされる。
 */
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct 牌(NonZeroU8);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 色 {
    萬 = 0,
    筒 = 1,
    索 = 2,
    中 = 3,
}

impl std::fmt::Display for 色 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::萬 => write!(f, "萬"),
            Self::筒 => write!(f, "筒"),
            Self::索 => write!(f, "索"),
            Self::中 => write!(f, "中"),
        }
    }
}

pub const 全牌一覧: [牌; 108 + 4] = unsafe {
    [
        /* 萬 */
        牌::from_id(0b00_0001_00),
        牌::from_id(0b00_0001_01),
        牌::from_id(0b00_0001_10),
        牌::from_id(0b00_0001_11),
        牌::from_id(0b00_0010_00),
        牌::from_id(0b00_0010_01),
        牌::from_id(0b00_0010_10),
        牌::from_id(0b00_0010_11),
        牌::from_id(0b00_0011_00),
        牌::from_id(0b00_0011_01),
        牌::from_id(0b00_0011_10),
        牌::from_id(0b00_0011_11),
        牌::from_id(0b00_0100_00),
        牌::from_id(0b00_0100_01),
        牌::from_id(0b00_0100_10),
        牌::from_id(0b00_0100_11),
        牌::from_id(0b00_0101_00),
        牌::from_id(0b00_0101_01),
        牌::from_id(0b00_0101_10),
        牌::from_id(0b00_0101_11),
        牌::from_id(0b00_0110_00),
        牌::from_id(0b00_0110_01),
        牌::from_id(0b00_0110_10),
        牌::from_id(0b00_0110_11),
        牌::from_id(0b00_0111_00),
        牌::from_id(0b00_0111_01),
        牌::from_id(0b00_0111_10),
        牌::from_id(0b00_0111_11),
        牌::from_id(0b00_1000_00),
        牌::from_id(0b00_1000_01),
        牌::from_id(0b00_1000_10),
        牌::from_id(0b00_1000_11),
        牌::from_id(0b00_1001_00),
        牌::from_id(0b00_1001_01),
        牌::from_id(0b00_1001_10),
        牌::from_id(0b00_1001_11),
        /* 筒 */
        牌::from_id(0b01_0001_00),
        牌::from_id(0b01_0001_01),
        牌::from_id(0b01_0001_10),
        牌::from_id(0b01_0001_11),
        牌::from_id(0b01_0010_00),
        牌::from_id(0b01_0010_01),
        牌::from_id(0b01_0010_10),
        牌::from_id(0b01_0010_11),
        牌::from_id(0b01_0011_00),
        牌::from_id(0b01_0011_01),
        牌::from_id(0b01_0011_10),
        牌::from_id(0b01_0011_11),
        牌::from_id(0b01_0100_00),
        牌::from_id(0b01_0100_01),
        牌::from_id(0b01_0100_10),
        牌::from_id(0b01_0100_11),
        牌::from_id(0b01_0101_00),
        牌::from_id(0b01_0101_01),
        牌::from_id(0b01_0101_10),
        牌::from_id(0b01_0101_11),
        牌::from_id(0b01_0110_00),
        牌::from_id(0b01_0110_01),
        牌::from_id(0b01_0110_10),
        牌::from_id(0b01_0110_11),
        牌::from_id(0b01_0111_00),
        牌::from_id(0b01_0111_01),
        牌::from_id(0b01_0111_10),
        牌::from_id(0b01_0111_11),
        牌::from_id(0b01_1000_00),
        牌::from_id(0b01_1000_01),
        牌::from_id(0b01_1000_10),
        牌::from_id(0b01_1000_11),
        牌::from_id(0b01_1001_00),
        牌::from_id(0b01_1001_01),
        牌::from_id(0b01_1001_10),
        牌::from_id(0b01_1001_11),
        /* 索 */
        牌::from_id(0b10_0001_00),
        牌::from_id(0b10_0001_01),
        牌::from_id(0b10_0001_10),
        牌::from_id(0b10_0001_11),
        牌::from_id(0b10_0010_00),
        牌::from_id(0b10_0010_01),
        牌::from_id(0b10_0010_10),
        牌::from_id(0b10_0010_11),
        牌::from_id(0b10_0011_00),
        牌::from_id(0b10_0011_01),
        牌::from_id(0b10_0011_10),
        牌::from_id(0b10_0011_11),
        牌::from_id(0b10_0100_00),
        牌::from_id(0b10_0100_01),
        牌::from_id(0b10_0100_10),
        牌::from_id(0b10_0100_11),
        牌::from_id(0b10_0101_00),
        牌::from_id(0b10_0101_01),
        牌::from_id(0b10_0101_10),
        牌::from_id(0b10_0101_11),
        牌::from_id(0b10_0110_00),
        牌::from_id(0b10_0110_01),
        牌::from_id(0b10_0110_10),
        牌::from_id(0b10_0110_11),
        牌::from_id(0b10_0111_00),
        牌::from_id(0b10_0111_01),
        牌::from_id(0b10_0111_10),
        牌::from_id(0b10_0111_11),
        牌::from_id(0b10_1000_00),
        牌::from_id(0b10_1000_01),
        牌::from_id(0b10_1000_10),
        牌::from_id(0b10_1000_11),
        牌::from_id(0b10_1001_00),
        牌::from_id(0b10_1001_01),
        牌::from_id(0b10_1001_10),
        牌::from_id(0b10_1001_11),
        /* 中 */
        牌::from_id(0b11_1111_00),
        牌::from_id(0b11_1111_01),
        牌::from_id(0b11_1111_10),
        牌::from_id(0b11_1111_11),
    ]
};

impl 牌 {
    /// # Safety
    /// id は合法な値であること
    #[must_use]
    pub const unsafe fn from_id(id: u8) -> Self {
        Self(NonZeroU8::new_unchecked(id))
    }

    #[must_use]
    pub fn 色(self) -> 色 {
        // u8 なので 6 bit 右シフトすることで残り 2bit。
        // 色は2bitの取りうる値すべてをカバーするのでOK。
        unsafe { std::mem::transmute(self.0.get() >> 6) }
    }

    #[must_use]
    pub fn 数(self) -> u8 {
        (self.0.get() >> 2) & 0b0000_1111
    }

    #[must_use]
    pub fn 詳細id(self) -> u8 {
        self.0.get() & 0b0000_0011
    }

    #[must_use]
    pub fn isワイルドカード(self) -> bool {
        self.0.get() >= 0b11_0000_00
    }

    #[must_use]
    pub fn is同一牌(self, other: Self) -> bool {
        (self.0.get() >> 2) == (other.0.get() >> 2)
    }

    #[must_use]
    pub fn is次牌of(self, other: Self) -> bool {
        (self.0.get() >> 2) == (other.0.get() >> 2) + 1
    }

    #[must_use]
    pub fn is次次牌of(self, other: Self) -> bool {
        (self.0.get() >> 2) == (other.0.get() >> 2) + 2
    }
}

impl Debug for 牌 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let 色 = self.色();
        let 数 = self.数();
        let id = self.詳細id();
        if 色 == 色::中 {
            write!(f, "{色}#{id}")
        } else {
            write!(f, "{数}{色}#{id}")
        }
    }
}

impl std::fmt::Display for 牌 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let 色 = self.色();
        let 数 = self.数();
        if 色 == 色::中 {
            write!(f, "{色}")
        } else {
            write!(f, "{数}{色}")
        }
    }
}
