use std::{ptr::{null_mut, self}};

use widestring::{U32String, u32str};

use fribidi_sys::fribidi_bindings::fribidi_remove_bidi_marks;

pub struct Fribidi;

impl Fribidi
{
    /// fribidi_remove_bidi_marks - remove bidi marks out of an string
    ///
    /// This function removes the bidi and boundary-neutral marks out of an string
    /// and the accompanying lists.  It implements rule X9 of the Unicode
    /// Bidirectional Algorithm available at
    /// http://www.unicode.org/reports/tr9/#X9, with the exception that it removes
    /// U+200E LEFT-TO-RIGHT MARK and U+200F RIGHT-TO-LEFT MARK too.
    ///
    /// If any of the input lists are NULL, the list is skipped.  If str is the
    /// visual string, then positions_to_this is  positions_L_to_V and
    /// position_from_this_list is positions_V_to_L;  if str is the logical
    /// string, the other way. Moreover, the position maps should be filled with
    /// valid entries.
    /// 
    /// A position map pointing to a removed character is filled with \(mi1. By the
    /// way, you should not use embedding_levels if str is visual string.
    /// 
    /// For best results this function should be run on a whole paragraph, not
    /// lines; but feel free to do otherwise if you know what you are doing.
    ///
    /// Returns: New length of the string, or \(mi1 if an error occurred (memory
    /// allocation failure most probably).
    ///
    pub fn remove_bidi_marks<'a>(
        input_str: &'a mut U32String,
        positions_to_this: Option<&mut Vec<i32>>,
        position_from_this_list: Option<&mut Vec<i32>>,
        embedding_levels: Option<&mut Vec<i8>>
    ) -> Result<&'a U32String, String>
    {
        let result_string_len = unsafe {
            fribidi_remove_bidi_marks(
                input_str.as_mut_ptr(),
                input_str.len() as i32,
                if let Some(positions) = positions_to_this {positions.as_mut_ptr()} else {null_mut()},
                if let Some(positions) = position_from_this_list {positions.as_mut_ptr()} else {null_mut()},
                if let Some(levels) = embedding_levels {levels.as_mut_ptr()} else {null_mut()}
            )
        };

        input_str.replace_range(result_string_len as usize.., u32str!(""));

        match result_string_len
        {
            -1 => Err("memory allocation failed".to_owned()),
            _ => Ok(input_str)
        }
    }
}

#[cfg(test)]
mod test
{
    use widestring::U32String;

    use super::{Fribidi, ParagraphType};

    #[test]
    fn test_remove_bidi_marks()
    {
        #[allow(text_direction_codepoint_in_literal)]
        let mut text = U32String::from("أحمد‫ خالد");
        let gt = U32String::from("أحمد خالد");

        let mut positions_to_this :Vec<i32> = vec![0; text.len()];
        let mut position_from_this_list :Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        assert_eq!(
            Fribidi::remove_bidi_marks(&mut text, Some(&mut positions_to_this), Some(&mut position_from_this_list), None),
            Ok(&gt)
        );

        assert_eq!(positions_to_this, vec![0, 1, 2, 3, -1, 4, 5, 6, 7, 9]);
        assert_eq!(position_from_this_list, vec![0, 1, 2, 3, 5, 6, 7, 8, 9, 9]);
    }
    }
}