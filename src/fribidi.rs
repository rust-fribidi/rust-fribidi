use std::{ptr::{null_mut, self}};

use widestring::{U32String, u32str};

use fribidi_sys::fribidi_bindings::fribidi_remove_bidi_marks;

pub struct Fribidi;

impl Fribidi
{
    /// remove_bidi_marks: remove bidi marks out of an string
    pub fn remove_bidi_marks<'a>(
        input_str: &'a mut U32String,
        positions_to_this: Option<&Vec<i32>>,
        position_from_this_list: Option<&Vec<i32>>,
        embedding_levels: Option<&str>
    ) -> Result<&'a U32String, String>
    {
        let result_string_len = unsafe {
            fribidi_remove_bidi_marks(
                input_str.as_mut_ptr(),
                input_str.len() as i32,
                if let Some(positions) = positions_to_this {ptr::read(positions.as_ptr()) as *mut _} else {null_mut()},
                if let Some(positions) = position_from_this_list {ptr::read(positions.as_ptr()) as *mut _} else {null_mut()},
                if let Some(levels) = embedding_levels {ptr::read(levels.as_ptr()) as *mut _} else {null_mut()}
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

    use super::Fribidi;

    #[test]
    fn test_remove_bidi_marks()
    {
        #[allow(text_direction_codepoint_in_literal)]
        let mut text = U32String::from("أحمد‫ خالد");
        let gt = U32String::from("أحمد خالد");

        assert_eq!(
            Fribidi::remove_bidi_marks(&mut text, None, None, None),
            Ok(&gt)
        );
    }
}