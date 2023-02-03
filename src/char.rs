use std::mem::transmute;

use fribidi_sys::fribidi_bindings;

type Char = u32;

#[repr(u32)]
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum CharType
{
    LeftToRight              = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_LTR,
    RightToLeft              = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_RTL,
    ArabicLetter             = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_AL,
    EuropeanNumeral          = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_EN,
    ArabicNumeral            = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_AN,
    EuropeanNumberSeparator  = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_ES,
    EuropeanNumberTerminator = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_ET,
    CommonSeparator          = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_CS,
    NonSpacingMark           = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_NSM,
    BoundaryNeutral          = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_BN,
    BlockSeparator           = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_BS,
    SegmentSeparator         = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_SS,
    WhiteSpace               = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_WS,
    OtherNeutral             = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_ON,
    LeftToRightEmbedding     = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_LRE,
    RightToLeftEmbedding     = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_RLE,
    LeftToRightOverride      = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_LRO,
    RightToLeftOverride      = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_RLO,
    PopDirectionalFlag       = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_PDF,
    LeftToRightIsolate       = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_LRI,
    RightToLeftIsolate       = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_RLI,
    FirstStongIsolate        = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_FSI,
    PopDirectionalIsolate    = fribidi_bindings::FriBidiCharType_FRIBIDI_TYPE_PDI,
}

impl TryFrom<u32> for CharType
{
    type Error = &'static str;

    fn try_from(raw: u32) -> Result<Self, Self::Error>
    {
        let res: CharType = unsafe { transmute(raw) };
        match res as u32
        {
            raw => Ok(res),
            _ => Err("Wrong u32 input")
        }
    }
}

impl TryFrom<char> for CharType
{
    type Error = &'static str;

    fn try_from(raw: char) -> Result<Self, Self::Error>
    {
        (raw as u32).try_into()
    }
}

impl Into<u32> for CharType
{
    fn into(self) -> u32 {
        self as u32
    }
}

impl CharType
{
    /// Is right to left: RTL, AL, RLE, RLO?
    pub fn is_rtl(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_RTL != 0
    }

    /// Is arabic: AL, AN?
    pub fn is_arabic(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_ARABIC != 0
    }

    // pub fn is_(ch: Char) -> bool
    // {
    //     ch & fribidi_bindings:: != 0
    // }

    pub fn is_strong(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_STRONG != 0
    }

    pub fn is_weak(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_WEAK != 0
    }

    pub fn is_netural(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_NEUTRAL != 0
    }

    pub fn is_sentinel(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_SENTINEL != 0
    }

    /// Is letter: L, R, AL?
    pub fn is_letter(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_LETTER != 0
    }

    /// Is number: EN, AN?
    pub fn is_number(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_NUMBER != 0
    }

    /// Is number separator or terminator: ES, ET, CS?
    pub fn is_number_separator_or_terminator(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_NUMSEPTER != 0
    }

    /// Is space: BN, BS, SS, WS?
    pub fn is_space(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_SPACE != 0
    }

    /// Is explicit mark: LRE, RLE, LRO, RLO, PDF?
    pub fn is_explicit(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_EXPLICIT != 0
    }

    pub fn is_isolate(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_ISOLATE != 0
    }

    /// Is text separator: BS, SS? 
    pub fn is_separator(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_SEPARATOR != 0
    }

    /// Is explicit override: LRO, RLO?
    pub fn is_override(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_OVERRIDE != 0
    }

    pub fn is_left_to_right_letter(ch: Char) -> bool
    {
        ch & (fribidi_bindings::FRIBIDI_MASK_LETTER | fribidi_bindings::FRIBIDI_MASK_RTL)
            == fribidi_bindings::FRIBIDI_MASK_LETTER
    }

    pub fn is_right_to_left_letter(ch: Char) -> bool
    {
        ch & (fribidi_bindings::FRIBIDI_MASK_LETTER | fribidi_bindings::FRIBIDI_MASK_RTL)
	        == (fribidi_bindings::FRIBIDI_MASK_LETTER | fribidi_bindings::FRIBIDI_MASK_RTL)
    }

    pub fn is_es_or_cs(ch: Char) -> bool
    {
        ch & (fribidi_bindings::FRIBIDI_MASK_ES | fribidi_bindings::FRIBIDI_MASK_CS) != 0
    }

    /// Is explicit or BN: LRE, RLE, LRO, RLO, PDF, BN?
    pub fn is_explicit_or_bn(ch: Char) -> bool
    {
        ch & (fribidi_bindings::FRIBIDI_MASK_EXPLICIT | fribidi_bindings::FRIBIDI_MASK_BN) != 0
    }

    /// Is explicit or BN or NSM: LRE, RLE, LRO, RLO, PDF, BN, NSM?
    pub fn is_explicit_or_bn_or_nsm(ch: Char) -> bool
    {
        ch & (fribidi_bindings::FRIBIDI_MASK_EXPLICIT | fribidi_bindings::FRIBIDI_MASK_BN | fribidi_bindings::FRIBIDI_MASK_NSM) != 0
    }

    /// Is explicit or BN or NSM: LRE, RLE, LRO, RLO, PDF, BN, NSM?
    pub fn is_explicit_or_isolate_or_bn_or_nsm(ch: Char) -> bool
    {
        ch & (fribidi_bindings::FRIBIDI_MASK_EXPLICIT
            | fribidi_bindings::FRIBIDI_MASK_ISOLATE
            | fribidi_bindings::FRIBIDI_MASK_BN
            | fribidi_bindings::FRIBIDI_MASK_NSM) != 0
    }

    /// Is explicit or BN or WS: LRE, RLE, LRO, RLO, PDF, BN, WS?
    pub fn is_explicit_or_bn_or_ws(ch: Char) -> bool
    {
        ch & (fribidi_bindings::FRIBIDI_MASK_EXPLICIT
            | fribidi_bindings::FRIBIDI_MASK_BN
            | fribidi_bindings::FRIBIDI_MASK_WS) != 0
    }

    /// Is explicit or separator or BN or WS: LRE, RLE, LRO, RLO, PDF, BS, SS, BN, WS?
    pub fn is_explicit_or_separator_or_bn_or_ws(ch: Char) -> bool
    {
        ch & (fribidi_bindings::FRIBIDI_MASK_EXPLICIT
            | fribidi_bindings::FRIBIDI_MASK_SEPARATOR
            | fribidi_bindings::FRIBIDI_MASK_BN
            | fribidi_bindings::FRIBIDI_MASK_WS) != 0
    }

    /// Is private-use type for application?
    pub fn is_private(ch: Char) -> bool
    {
        ch & fribidi_bindings::FRIBIDI_MASK_PRIVATE != 0
    }

}