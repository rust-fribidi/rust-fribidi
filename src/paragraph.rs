use fribidi_sys::fribidi_bindings;

use crate::char::CharType;

pub type Paragraph = u32;

#[repr(u32)]
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum ParagraphType
{
    LeftToRight     = fribidi_bindings::FriBidiParType_FRIBIDI_PAR_LTR,
    RightToLeft     = fribidi_bindings::FriBidiParType_FRIBIDI_PAR_RTL,
    OtherNeutral    = fribidi_bindings::FriBidiParType_FRIBIDI_PAR_ON,
    WeakLeftToRight = fribidi_bindings::FriBidiParType_FRIBIDI_PAR_WLTR,
    WeakRightToLeft = fribidi_bindings::FriBidiParType_FRIBIDI_PAR_WRTL,
}

impl ParagraphType
{
    /// same as `fribidi_get_par_direction` - get base paragraph direction
    ///
    /// This function finds the base direction of a single paragraph,
    /// as defined by rule P2 of the Unicode Bidirectional Algorithm available at
    /// http://www.unicode.org/reports/tr9/#P2.
    ///
    /// You typically do not need this function as
    /// fribidi_get_par_embedding_levels() knows how to compute base direction
    /// itself, but you may need this to implement a more sophisticated paragraph
    /// direction handling.  Note that you can pass more than a paragraph to this
    /// function and the direction of the first non-neutral paragraph is returned,
    /// which is a very good heuristic to set direction of the neutral paragraphs
    /// at the beginning of text.  For other neutral paragraphs, you better use the
    /// direction of the previous paragraph.
    ///
    /// Returns: Base pargraph direction.  No weak paragraph direction is returned,
    /// only LeftToRight, RightToLeft, or OtherNeutral.
    ///
    pub fn direction (char_types: &Vec<CharType>) -> ParagraphType
    {
        let par_direction = unsafe {
            // let bidi_type_name = fribidi_bindings::fribidi_get_bidi_type_name(char_type as u32);
            // fribidi_bindings::fribidi_get_par_direction(bidi_type_name, len);
            fribidi_bindings::fribidi_get_par_direction(char_types.as_ptr() as *const u32, char_types.len() as i32)
        };

        unsafe { std::mem::transmute (par_direction) }
    }

    /// Weaken type for paragraph fallback purposes:
    /// LTR->WLTR, RTL->WRTL.
    pub fn weak(ch: Paragraph) -> bool
    {
        Self::WeakRightToLeft as u32 | (ch & fribidi_bindings::FRIBIDI_MASK_RTL) != 0
    }
}

#[cfg(test)]
mod test
{
    use crate::{char::CharType, paragraph::ParagraphType};

    #[test]
    fn test_direction ()
    {
        let char_types = vec![CharType::ArabicLetter, CharType::ArabicNumeral, CharType::LeftToRight, CharType::LeftToRight];
        let gt = ParagraphType::RightToLeft;

        let par_dir = ParagraphType::direction(&char_types);

        assert_eq!(par_dir, gt);
    }
}