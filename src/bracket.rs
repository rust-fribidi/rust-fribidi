use fribidi_sys::fribidi_bindings;

use widestring::U32String;

use crate::CharType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bracket {
    btype: u32
}

impl From<u32> for Bracket
{
    fn from(raw: u32) -> Self {
        Self { btype: raw }
    }
}

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
impl From<char> for Bracket
{
    fn from(raw: char) -> Self {
        Self { btype: raw as u32 }
    }
}

impl Into<u32> for Bracket
{
    fn into(self) -> u32 {
        self.btype
    }
}
    
impl Bracket
{
    /// new - get bracketed character
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
    pub fn from_char (ch: char) -> Bracket
    {
        Self::from_u32(ch as u32)
    }

    pub fn from_u32 (raw: u32) -> Bracket
    {
        let raw_btype = unsafe {
            fribidi_bindings::fribidi_get_bracket(raw)
        };

        Self { btype: raw_btype }
    }

    pub fn is_open(&self) -> bool
    {
        (self.btype & fribidi_bindings::FRIBIDI_BRACKET_OPEN_MASK) > 0
    }

    pub fn get_id(&self) -> u32
    {
        self.btype & fribidi_bindings::FRIBIDI_BRACKET_ID_MASK
    }

    /// parse - get bracketed characters
    ///
    /// This function finds the bracketed characters of an string of characters.
    /// See fribidi_get_bracket() for more information about the bracketed
    /// characters returned by this function.
    ///
    pub fn parse (
        input_str: &U32String,
        char_types: &Vec<CharType>
    ) -> Vec<Bracket>
    {
        let mut bracket_types: Vec<Bracket> = vec![0.into(); input_str.len()];
        unsafe {
            fribidi_bindings::fribidi_get_bracket_types(
                input_str.as_ptr(),
                input_str.len() as i32,
                char_types.as_ptr() as *const u32,
                bracket_types.as_mut_ptr() as *mut u32
            )
        };

        bracket_types
    }
}

#[cfg(test)]
mod test
{
    use widestring::U32String;

    use crate::bracket::Bracket;

    #[test]
    fn test_get_bracket()
    {
        let bracketed_char: Bracket = ']'.into();
        let gt = '['.into();

        assert_eq!(bracketed_char, gt);
    }

    #[test]
    fn test_get_bracket_types()
    {
        let text = U32String::from("[{][أحمد)");
        let char_types = crate::Fribidi::get_bidi_types(&text);
        let bracket_types = Bracket::parse(&text, &char_types);
        let gt: Vec<Bracket> = [2147483739 as u32, 2147483771, 91, 2147483739, 0, 0, 0, 0, 40]
            .iter()    
            .map(|n| (*n).into())
            .collect();

        assert_eq!(bracket_types, gt);
    }
}