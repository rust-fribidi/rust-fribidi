use std::mem::transmute;

use fribidi_sys::fribidi_bindings;
use crate::level::LevelType;

pub type Char = u32;

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

/// BUG
impl From<Char> for CharType
{
    fn from(raw: Char) -> Self
    {
        let char_type = unsafe {
            transmute (
                fribidi_bindings::fribidi_get_bidi_type(raw as Char)
            )
        };

        char_type
    }
}

impl From<char> for CharType
{
    fn from(raw: char) -> Self
    {
        (raw as Char).into()
    }
}

impl CharType
{
    /// fribidi_get_bidi_type - get character bidi type
    ///
    /// This function returns the bidi type of a character as defined in Table 3.7
    /// Bidirectional Character Types of the Unicode Bidirectional Algorithm
    /// available at
    /// http://www.unicode.org/reports/tr9/#Bidirectional_Character_Types, using
    /// data provided in file UnicodeData.txt of the Unicode Character Database
    /// available at http://www.unicode.org/Public/UNIDATA/UnicodeData.txt.
    ///
    /// There are a few macros defined in fribidi-bidi-types.h for querying a bidi
    /// type.
    ///
    pub fn into_chartype(chartype: Char) -> CharType
    {
        chartype.into()
    }

    /// fribidi_get_bidi_types - get bidi types for an string of characters
    ///
    /// This function finds the bidi types of an string of characters.  See
    /// fribidi_get_bidi_type() for more information about the bidi types returned
    /// by this function.
    ///
    pub fn into_chartypes(chartypes: &Vec<Char>) -> Vec<CharType>
    {
        chartypes
            .iter()
            .map(|&ch| (ch as Char).into())
            .collect()
    }

    /// fribidi_get_bidi_type_name - get bidi type name
    ///
    /// This function returns the bidi type name of a character type.
    ///
    /// The type names are the same as ones defined in Table 3.7 Bidirectional
    /// Character Types of the Unicode Bidirectional Algorithm available at
    /// http://www.unicode.org/reports/tr9/#Bidirectional_Character_Types, with a
    /// few modifications: L->LTR, R->RTL, B->BS, S->SS.
    ///
    pub fn name (char_type: CharType) -> String
    {
        // unsafe {
        //     let bidi_type_name = fribidi_bindings::fribidi_get_bidi_type_name(char_type as u32);
        //     std::ffi::CStr::from_ptr(bidi_type_name).to_str().unwrap()
        // }

        format!("{:?}", char_type)
    }

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

    /// Change numbers to RTL: EN,AN -> RTL.
    pub fn into_right_to_left(ch: Char) -> CharType
    {
        match Self::is_number(ch) {
            true => CharType::RightToLeft,
            false => ch.into()
        }
    }

    /// Override status of an explicit mark:
    /// LRO,LRE->LTR, RLO,RLE->RTL, otherwise->ON.
    pub fn explicit_to_override_dir(ch: Char) -> CharType
    {
        match Self::is_override(ch) {
            true => LevelType::to_chartype(&LevelType::from_char(ch)),
		    false => CharType::OtherNeutral
        }
    }
}

mod test
{
    use widestring::U32String;
    use crate::char::CharType;

    #[test]
    fn test_type ()
    {
        let ch = 'غ';
        let ch_type: Result<CharType, _> = ch.try_into();
        // assert!(ch_type.is_ok());
        let gt = CharType::ArabicLetter;
        
        assert_eq!(ch_type.unwrap(), gt);
    }

    #[test]
    fn test_get_bidi_types ()
    {
        let text = U32String::from("غ!A西Б1٤");
        let types = CharType::into_chartypes(text.as_vec());
        // assert!(types.is_ok());
        let gt = vec![
            CharType::ArabicLetter,
            CharType::OtherNeutral,
            CharType::LeftToRight,
            CharType::LeftToRight,
            CharType::LeftToRight,
            CharType::EuropeanNumeral,
            CharType::ArabicNumeral
        ];
        
        assert_eq!(types, gt);
    }

    #[test]
    fn test_get_bidi_type_name ()
    {
        let char_type = CharType::EuropeanNumberSeparator;
        let gt = "EuropeanNumberSeparator".to_owned();

        let char_type_name = CharType::name(char_type);
        assert_eq!(char_type_name, gt);
    }
}