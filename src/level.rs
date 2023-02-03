use crate::{CharType, char::Char};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LevelType(pub i8);

impl From<i8> for LevelType
{
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl LevelType
{
    pub fn is_right_to_left(&self) -> bool
    {
        self.0 & 1 != 0
    }

    /// Return the bidi type corresponding to the direction of the level number,
    /// FRIBIDI_TYPE_LTR for evens and FRIBIDI_TYPE_RTL for odds.
    pub fn to_chartype(&self) -> CharType
    {
        match self.is_right_to_left()
        {
            true => CharType::RightToLeft,
            false => CharType::LeftToRight
        }
    }

    /// Return the minimum level of the direction, 0 for FRIBIDI_TYPE_LTR and
    /// 1 for FRIBIDI_TYPE_RTL and FRIBIDI_TYPE_AL.
    pub fn from_chartype(chartype: CharType) -> Self
    {
        if chartype as u32 & 1 != 0
        {
            Self(1)
        }
        else
        {
            Self(0)
        }
    }

    /// Return the minimum level of the direction, 0 for FRIBIDI_TYPE_LTR and
    /// 1 for FRIBIDI_TYPE_RTL and FRIBIDI_TYPE_AL.
    pub fn from_char(chartype: Char) -> Self
    {
        if chartype as u32 & 1 != 0
        {
            Self(1)
        }
        else
        {
            Self(0)
        }
    }
}