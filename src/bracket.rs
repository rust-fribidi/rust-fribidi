use fribidi_sys::fribidi_bindings;

use std::mem::transmute;

pub struct Bracket {
    char: u32
}

#[repr(u32)]
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum BracketType
{
    NoBrackets = fribidi_bindings::FRIBIDI_NO_BRACKET,
    LeftParenthesis = 0x0029, // open
    RightParenthesis = 0x0028, // close
    LeftSquareBracket = 0x005D, // open
    RightSquareBracket = 0x005B, // close
    LeftCurlyBracket = 0x007D, // open
    RightCurlyBracket = 0x007B, // close
    TibetanMarkGugRtagsGyon = 0x0F3B, // open
    TibetanMarkGugRtagsGyas = 0x0F3A, // close
    TibetanMarkAngKhangGyon = 0x0F3D, // open
    TibetanMarkAngKhangGyas = 0x0F3C, // close
    OghamFeatherMark = 0x169C, // open
    OghamReversedFeatherMark = 0x169B, // close
    LeftSquareBracketWithQuill = 0x2046, // open
    RightSquareBracketWithQuill = 0x2045, // close
    SuperscriptLeftParenthesis = 0x207E, // open
    SuperscriptRightParenthesis = 0x207D, // close
    SubscriptLeftParenthesis = 0x208E, // open
    SubscriptRightParenthesis = 0x208D, // close
    LeftCeiling = 0x2309, // open
    RightCeiling = 0x2308, // close
    LeftFloor = 0x230B, // open
    RightFloor = 0x230A, // close
    LeftPointingAngleBracket = 0x232A, // open
    RightPointingAngleBracket = 0x2329, // close
    MediumLeftParenthesisOrnament = 0x2769, // open
    MediumRightParenthesisOrnament = 0x2768, // close
    MediumFlattenedLeftParenthesisOrnament = 0x276B, // open
    MediumFlattenedRightParenthesisOrnament = 0x276A, // close
    MediumLeftPointingAngleBracketOrnament = 0x276D, // open
    MediumRightPointingAngleBracketOrnament = 0x276C, // close
    HeavyLeftPointingAngleQuotationMarkOrnament = 0x276F, // open
    HeavyRightPointingAngleQuotationMarkOrnament = 0x276E, // close
    HeavyLeftPointingAngleBracketOrnament = 0x2771, // open
    HeavyRightPointingAngleBracketOrnament = 0x2770, // close
    LightLeftTortoiseShellBracketOrnament = 0x2773, // open
    LightRightTortoiseShellBracketOrnament = 0x2772, // close
    MediumLeftCurlyBracketOrnament = 0x2775, // open
    MediumRightCurlyBracketOrnament = 0x2774, // close
    LeftSShapedBagDelimiter = 0x27C6, // open
    RightSShapedBagDelimiter = 0x27C5, // close
    MathematicalLeftWhiteSquareBracket = 0x27E7, // open
    MathematicalRightWhiteSquareBracket = 0x27E6, // close
    MathematicalLeftAngleBracket = 0x27E9, // open
    MathematicalRightAngleBracket = 0x27E8, // close
    MathematicalLeftDoubleAngleBracket = 0x27EB, // open
    MathematicalRightDoubleAngleBracket = 0x27EA, // close
    MathematicalLeftWhiteTortoiseShellBracket = 0x27ED, // open
    MathematicalRightWhiteTortoiseShellBracket = 0x27EC, // close
    MathematicalLeftFlattenedParenthesis = 0x27EF, // open
    MathematicalRightFlattenedParenthesis = 0x27EE, // close
    LeftWhiteCurlyBracket = 0x2984, // open
    RightWhiteCurlyBracket = 0x2983, // close
    LeftWhiteParenthesis = 0x2986, // open
    RightWhiteParenthesis = 0x2985, // close
    ZNotationLeftImageBracket = 0x2988, // open
    ZNotationRightImageBracket = 0x2987, // close
    ZNotationLeftBindingBracket = 0x298A, // open
    ZNotationRightBindingBracket = 0x2989, // close
    LeftSquareBracketWithUnderbar = 0x298C, // open
    RightSquareBracketWithUnderbar = 0x298B, // close
    LeftSquareBracketWithTickInTopCorner = 0x2990, // open
    RightSquareBracketWithTickInBottomCorner = 0x298F, // close
    LeftSquareBracketWithTickInBottomCorner = 0x298E, // open
    RightSquareBracketWithTickInTopCorner = 0x298D, // close
    LeftAngleBracketWithDot = 0x2992, // open
    RightAngleBracketWithDot = 0x2991, // close
    LeftArcLessThanBracket = 0x2994, // open
    RightArcGreaterThanBracket = 0x2993, // close
    DoubleLeftArcGreaterThanBracket = 0x2996, // open
    DoubleRightArcLessThanBracket = 0x2995, // close
    LeftBlackTortoiseShellBracket = 0x2998, // open
    RightBlackTortoiseShellBracket = 0x2997, // close
    LeftWigglyFence = 0x29D9, // open
    RightWigglyFence = 0x29D8, // close
    LeftDoubleWigglyFence = 0x29DB, // open
    RightDoubleWigglyFence = 0x29DA, // close
    LeftPointingCurvedAngleBracket = 0x29FD, // open
    RightPointingCurvedAngleBracket = 0x29FC, // close
    TopLeftHalfBracket = 0x2E23, // open
    TopRightHalfBracket = 0x2E22, // close
    BottomLeftHalfBracket = 0x2E25, // open
    BottomRightHalfBracket = 0x2E24, // close
    LeftSidewaysUBracket = 0x2E27, // open
    RightSidewaysUBracket = 0x2E26, // close
    LeftDoubleParenthesis = 0x2E29, // open
    RightDoubleParenthesis = 0x2E28, // close
    LeftSquareBracketWithStroke = 0x2E56, // open
    RightSquareBracketWithStroke = 0x2E55, // close
    LeftSquareBracketWithDoubleStroke = 0x2E58, // open
    RightSquareBracketWithDoubleStroke = 0x2E57, // close
    TopHalfLeftParenthesis = 0x2E5A, // open
    TopHalfRightParenthesis = 0x2E59, // close
    BottomHalfLeftParenthesis = 0x2E5C, // open
    BottomHalfRightParenthesis = 0x2E5B, // close
    LeftAngleBracket = 0x3009, // open
    RightAngleBracket = 0x3008, // close
    LeftDoubleAngleBracket = 0x300B, // open
    RightDoubleAngleBracket = 0x300A, // close
    LeftCornerBracket = 0x300D, // open
    RightCornerBracket = 0x300C, // close
    LeftWhiteCornerBracket = 0x300F, // open
    RightWhiteCornerBracket = 0x300E, // close
    LeftBlackLenticularBracket = 0x3011, // open
    RightBlackLenticularBracket = 0x3010, // close
    LeftTortoiseShellBracket = 0x3015, // open
    RightTortoiseShellBracket = 0x3014, // close
    LeftWhiteLenticularBracket = 0x3017, // open
    RightWhiteLenticularBracket = 0x3016, // close
    LeftWhiteTortoiseShellBracket = 0x3019, // open
    RightWhiteTortoiseShellBracket = 0x3018, // close
    LeftWhiteSquareBracket = 0x301B, // open
    RightWhiteSquareBracket = 0x301A, // close
    SmallLeftParenthesis = 0xFE5A, // open
    SmallRightParenthesis = 0xFE59, // close
    SmallLeftCurlyBracket = 0xFE5C, // open
    SmallRightCurlyBracket = 0xFE5B, // close
    SmallLeftTortoiseShellBracket = 0xFE5E, // open
    SmallRightTortoiseShellBracket = 0xFE5D, // close
    FullwidthLeftParenthesis = 0xFF09, // open
    FullwidthRightParenthesis = 0xFF08, // close
    FullwidthLeftSquareBracket = 0xFF3D, // open
    FullwidthRightSquareBracket = 0xFF3B, // close
    FullwidthLeftCurlyBracket = 0xFF5D, // open
    FullwidthRightCurlyBracket = 0xFF5B, // close
    FullwidthLeftWhiteParenthesis = 0xFF60, // open
    FullwidthRightWhiteParenthesis = 0xFF5F, // close
    HalfwidthLeftCornerBracket = 0xFF63, // open
    HalfwidthRightCornerBracket = 0xFF62, // close
}
    
impl Bracket
{
    pub fn is_bracket_open(bracket_char: Bracket) -> bool
    {
        (bracket_char.char as u32 & fribidi_bindings::FRIBIDI_BRACKET_OPEN_MASK) > 0
    }

    /// fribidi_get_bracket - get bracketed character
    ///
    /// This function finds the bracketed equivalent of a character as defined in
    /// the file BidiBrackets.txt of the Unicode Character Database available at
    /// http://www.unicode.org/Public/UNIDATA/BidiBrackets.txt.
    ///
    /// If  the input character is a declared as a brackets character in the
    /// Unicode standard and has a bracketed equivalent.  The matching bracketed
    /// character is put in the output, otherwise the input character itself is
    /// put.
    ///
    /// Returns: The bracket type of the character. Use the
    /// FRIBIDI_IS_BRACKET(FriBidiBracketType) to test if it is a valid
    /// property.
    ///
    pub fn get_bracket (ch: char) -> BracketType
    {
        fn bracket_id(bracket_char: Bracket) -> u32
        {
            bracket_char.char as u32 & fribidi_bindings::FRIBIDI_BRACKET_ID_MASK
        }

        unsafe {
            let bracketed_char = bracket_id(
                Bracket { char: fribidi_bindings::fribidi_get_bracket(ch as u32) }
            );
            transmute(bracketed_char)
        }
    }
}

#[cfg(test)]
mod test
{
    use crate::bracket::{BracketType, Bracket};

    #[test]
    fn test_get_bracket()
    {
        let ch = '[';
        let bracketed_char = Bracket::get_bracket(ch);
        let gt = BracketType::RightSquareBracket;

        assert_eq!(bracketed_char, gt);
    }

    #[test]
    fn test_get_par_embedding_levels_ex ()
    {
        unimplemented!()
    }
}