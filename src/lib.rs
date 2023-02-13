//! # Getting started
//!
//! ```rust,no_run
//! let text = U32String::from("چرمهين");
//! 
//! let gt = U32String::from("ﻦﻴﻬﻣﺮﭼ");
//! let gt_maximum_level = 2;
//! let gt_positions_logic_to_visual = vec![5, 4, 3, 2, 1, 0];
//! let gt_positions_visual_to_logic = vec![5, 4, 3, 2, 1, 0];
//! let gt_embedding_levels = vec![LevelType(1); 6];
//! 
//! let mut positions_logic_to_visual :Vec<i32> = vec![1; text.len()];
//! let mut positions_visual_to_logic :Vec<i32> = vec![1; text.len()];
//! let mut embedding_levels: Vec<LevelType> = vec![LevelType(1); text.len()];
//! 
//! let (res, maximum_level) = Fribidi::logic_to_visual(
//!     &text,
//!     ParagraphType::OtherNeutral,    // let fribidi detect the type
//!     Some(&mut positions_logic_to_visual),
//!     Some(&mut positions_visual_to_logic),
//!     Some(&mut embedding_levels)
//! ).unwrap();
//! 
//! assert_eq!((res, maximum_level), (gt, gt_maximum_level));
//! assert_eq!(positions_logic_to_visual, gt_positions_logic_to_visual);
//! assert_eq!(positions_visual_to_logic, gt_positions_visual_to_logic);
//! assert_eq!(embedding_levels, gt_embedding_levels);
//! ```

use std::ptr::{null_mut, null};

use widestring::{U32String, u32str, U32Str};

use fribidi_sys::fribidi_bindings;

pub mod bracket;
use bracket::BracketType;
pub mod char;
use crate::char::CharType;
pub mod level;
use level::LevelType;
pub mod paragraph;
use paragraph::ParagraphType;
pub mod flag;
use flag::FriBidiFlag;

pub struct Fribidi;
impl Fribidi
{
    /// same_as `fribidi_remove_bidi_marks` - remove bidi marks out of an string
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
    /// Returns: the input string after removing the marks.
    ///
    pub fn remove_bidirectional_marks<'a>(
        input_str: &'a mut U32String,
        positions_to_this: Option<&mut Vec<i32>>,
        position_from_this_list: Option<&mut Vec<i32>>,
        embedding_levels: Option<&mut Vec<LevelType>>
    ) -> Result<&'a U32String, String>
    {
        let result_string_len = unsafe {
            fribidi_bindings::fribidi_remove_bidi_marks(
                input_str.as_mut_ptr(),
                input_str.len() as i32,
                if let Some(positions) = positions_to_this {positions.as_mut_ptr()} else {null_mut()},
                if let Some(positions) = position_from_this_list {positions.as_mut_ptr()} else {null_mut()},
                if let Some(levels) = embedding_levels {levels.as_mut_ptr() as *mut i8} else {null_mut()}
            )
        };

        input_str.replace_range(result_string_len as usize.., u32str!(""));

        match result_string_len
        {
            -1 => Err("memory allocation failed".to_owned()),
            _ => Ok(input_str)
        }
    }

    /// same_as `fribidi_log`2vis - get visual string
    ///
    /// This function converts the logical input string to the visual output
    /// strings as specified by the Unicode Bidirectional Algorithm.  As a side
    /// effect it also generates mapping lists between the two strings, and the
    /// list of embedding levels as defined by the algorithm.
    ///
    /// If NULL is passed as any of the the lists, the list is ignored and not
    /// filled.
    ///
    /// Note that this function handles one-line paragraphs. For multi-
    /// paragraph texts it is necessary to first split the text into
    /// separate paragraphs and then carry over the resolved pbase_dir
    /// between the subsequent invocations.
    ///
    /// Returns: visual result string and the maximum level found plus one, or zero if any error occurred
    /// (memory allocation failure most probably).
    ///
    pub fn logic_to_visual(
        input_str: &U32String,
        paragraph_direction: ParagraphType,
        positions_logic_to_visual: Option<&mut Vec<i32>>,
        positions_visual_to_logic: Option<&mut Vec<i32>>,
        embedding_levels: Option<&mut Vec<LevelType>>
    ) -> Result<(U32String, i8), String>
    {
        let mut visual_str = std::iter::repeat(" ").take(input_str.len()).collect::<U32String>();

        let maximum_level = unsafe {
            fribidi_bindings::fribidi_log2vis(
                input_str.as_ptr(),
                input_str.len() as i32,
                &mut (paragraph_direction as u32),
                visual_str.as_mut_ptr(),
                if let Some(positions) = positions_logic_to_visual {positions.as_mut_ptr()} else {null_mut()},
                if let Some(positions) = positions_visual_to_logic {positions.as_mut_ptr()} else {null_mut()},
                if let Some(levels) = embedding_levels {levels.as_mut_ptr() as *mut i8} else {null_mut()}
            )
        };

        match maximum_level
        {
            0 => Err("memory allocation failed".to_owned()),
            _ => Ok((visual_str, maximum_level))
        }
    }

    /// same_as `fribidi_get_par_embedding_levels_ex` - get bidi embedding levels of a paragraph
    ///
    /// This function finds the bidi embedding levels of a single paragraph,
    /// as defined by the Unicode Bidirectional Algorithm available at
    /// http://www.unicode.org/reports/tr9/.  This function implements rules P2 to
    /// I1 inclusive, and parts 1 to 3 of L1, except for rule X9 which is
    ///  implemented in fribidi_remove_bidi_marks().  Part 4 of L1 is implemented
    ///  in fribidi_reorder_line().
    ///
    /// There are a few macros defined in fribidi-bidi-types.h to work with this
    /// embedding levels.
    ///
    /// Returns: 
    ///     0: the result embedding levels
    ///     1: Maximum level found plus one, or zero if any error occurred
    ///        (memory allocation failure most probably).
    ///     2: the result ParagraphType
    ///
    // FRIBIDI_ENTRY FriBidiLevel
    pub fn get_paragraph_embedding_levels_ex (
        char_types: &Vec<CharType>,
        bracket_types: Option<&Vec<BracketType>>,
        paragraph_direction: ParagraphType
    ) -> Result<(Vec<LevelType>, LevelType, ParagraphType), String>
    {
        if bracket_types.is_some()
        {
            if char_types.len() != bracket_types.map_or(char_types.len(), |types| types.len())
            {
                return Err("char_types length must equals bracket_types length".to_owned());
            }
        }

        let mut res: Vec<LevelType> = vec![LevelType(0); char_types.len()];
        let paragraph_direction: ParagraphType = paragraph_direction;

        let max_embedding_level = unsafe {
            fribidi_bindings::fribidi_get_par_embedding_levels_ex (
                char_types.as_ptr() as *const u32,
                if let Some(types) = bracket_types { types.as_ptr() as *const u32 } else { null() },
                char_types.len() as i32,
                &mut (paragraph_direction as u32),
                res.as_mut_ptr() as *mut i8
            )
        };

        match max_embedding_level
        {
            0 => Err("memory allocation failed".to_owned()),
            _ => Ok((res, LevelType(max_embedding_level), paragraph_direction))
        }
    }

    /// same_as `fribidi_reorder_line` - reorder a line of logical string to visual
    ///
    /// This function reorders the characters in a line of text from logical to
    /// final visual order.  This function implements part 4 of rule L1, and rules
    /// L2 and L3 of the Unicode Bidirectional Algorithm available at
    /// http://www.unicode.org/reports/tr9/#Reordering_Resolved_Levels.
    ///
    /// As a side effect it also sets position maps if not NULL.
    ///
    /// You should provide the resolved paragraph direction and embedding levels as
    /// set by get_paragraph_embedding_levels().  Also note that the embedding
    /// levels may change a bit.  To be exact, the embedding level of any sequence
    /// of white space at the end of line is reset to the paragraph embedding level
    /// (That is part 4 of rule L1).
    ///
    /// Note that the bidi types and embedding levels are not reordered.  You can
    /// reorder these (or any other) arrays using the map later.  The user is
    /// responsible to initialize map to something sensible, like an identity
    /// mapping, or pass NULL if no map is needed.
    ///
    /// There is an optional part to this function, which is whether non-spacing
    /// marks for right-to-left parts of the text should be reordered to come after
    /// their base characters in the visual string or not.  Most rendering engines
    /// expect this behavior, but console-based systems for example do not like it.
    /// This is controlled by the FRIBIDI_FLAG_REORDER_NSM flag.  The flag is on
    /// in FRIBIDI_FLAGS_DEFAULT.
    ///
    /// Returns: Maximum level found in this line plus one, or zero if any error
    /// occurred (memory allocation failure most probably) and the reordered result line.
    ///
    pub fn reorder_line(
        flags: FriBidiFlag,                             // reorder flags
        chartypes: &Vec<CharType>,	                    // input list of bidi types as returned by fribidi_get_bidi_types()
        base_dir: ParagraphType,	                    // resolved paragraph base direction
        embedding_levels: Option<&mut Vec<LevelType>>,	// input list of embedding levels, as returned by fribidi_get_par_embedding_levels
        visual_str: &mut U32Str,	                    // visual string to reorder
    ) -> Result<(LevelType, Vec<u32>), &'static str>    // a map of string indices which is reordered to reflect where each glyph ends up.
    {
        let embedding_levels_len = embedding_levels.as_ref().map_or(visual_str.len(), |levels| levels.len());
        if chartypes.len() != embedding_levels_len || chartypes.len() != visual_str.len()
        {
            return Err("chartypes.len() != embedding_levels.len() != visual_str.len()");
        }

        // let mut res: Vec<usize> = vec![1; chartypes.len()];
        let mut res_map: Vec<u32> = (0..visual_str.len() as u32).collect();

        let max_level = unsafe {
            fribidi_bindings::fribidi_reorder_line(
                flags as u32,
                chartypes.as_ptr() as *const u32,
                visual_str.len() as i32,
                0,
                base_dir as u32,
                // if let Some(levels) = embedding_levels { levels.as_mut_ptr() as *mut i8 } else { null_mut() },
                embedding_levels.map_or(null_mut(), |levels| levels.as_mut_ptr() as *mut i8),
                visual_str.as_mut_ptr(),
                res_map.as_mut_ptr() as *mut i32
            )
        };

        match max_level
        {
            0 => Err("memory allocation failed"),
            _ => Ok((max_level.into(), res_map))
        }
    }
  
}

#[cfg(test)]
mod test
{
    use widestring::U32String;

    #[allow(unused_imports)]
    use crate::BracketType;
    use crate::flag::FriBidiFlag;

    use super::{Fribidi, ParagraphType, CharType, LevelType};

    #[test]
    fn test_remove_bidi_marks()
    {
        #[allow(text_direction_codepoint_in_literal)]
        let mut text = U32String::from("أحمد‫ خالد");
        let gt = U32String::from("أحمد خالد");

        let mut positions_to_this :Vec<i32> = vec![0; text.len()];
        let mut position_from_this_list :Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        assert_eq!(
            Fribidi::remove_bidirectional_marks(&mut text, Some(&mut positions_to_this), Some(&mut position_from_this_list), None),
            Ok(&gt)
        );

        assert_eq!(positions_to_this, vec![0, 1, 2, 3, -1, 4, 5, 6, 7, 9]);
        assert_eq!(position_from_this_list, vec![0, 1, 2, 3, 5, 6, 7, 8, 9, 9]);
    }

    #[test]
    fn test_log2vis()
    {
        let text = U32String::from("چرمهين");
        let gt = (U32String::from("ﻦﻴﻬﻣﺮﭼ"), 2);
        let gt_positions_l_to_v = vec![5, 4, 3, 2, 1, 0];
        let gt_positions_v_to_l = vec![5, 4, 3, 2, 1, 0];
        let gt_embedding_levels = vec![LevelType(1); 6];

        let mut positions_l_to_v :Vec<i32> = vec![1; text.len()];
        let mut positions_v_to_l :Vec<i32> = vec![1; text.len()];
        let mut embedding_levels: Vec<LevelType> = vec![LevelType(1); text.len()];

        let (res, maximum_level) = Fribidi::logic_to_visual(
            &text,
            ParagraphType::OtherNeutral,    // let fribidi detect the type
            Some(&mut positions_l_to_v),
            Some(&mut positions_v_to_l),
            Some(&mut embedding_levels)
        ).unwrap();

        assert_eq!((res, maximum_level), gt);
        assert_eq!(positions_l_to_v, gt_positions_l_to_v);
        assert_eq!(positions_v_to_l, gt_positions_v_to_l);
        assert_eq!(embedding_levels, gt_embedding_levels);
    }

    #[test]
    fn test_get_paragraph_embedding_levels_ex ()
    {
        let text = U32String::from("(أحمد خالد 比 توفـــــيق boieng 1997)");
        let char_types = CharType::into_chartypes(text.as_vec());
        let bracket_types = BracketType::parse(&text, &char_types);
        let paragraph_dir = ParagraphType::direction(&char_types);

        let res = Fribidi::get_paragraph_embedding_levels_ex(
            &char_types,
            Some(&bracket_types),
            paragraph_dir
        );
        assert!(res.is_ok());
        let (embedding_levels, max_embedding_level, paragraph_type) = res.unwrap();

        let gt_embedding_levels: Vec<LevelType> = 
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]
                .iter()
                .map(|&elm| LevelType(elm))
                .collect();
        let gt_max_embedding_level = LevelType(3);
        let gt_paragraph_type = ParagraphType::RightToLeft;

        assert_eq!(embedding_levels, gt_embedding_levels);
        assert_eq!(max_embedding_level, gt_max_embedding_level);
        assert_eq!(paragraph_type, gt_paragraph_type);
    }

    #[test]
    fn test_reorder_line()
    {
        let mut text = U32String::from(")دلاخ boieng 1997 قيـــــفوت 比 دمحأ(");
        let gt = U32String::from("(أحمد 比 توفـــــيق boieng 1997 خالد)");
        let gt_map = vec![35, 34, 33, 32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 5, 4, 3, 2, 1, 0];

        let char_types = CharType::into_chartypes(text.as_vec());
        let paragraph_direction = ParagraphType::direction(&char_types);
        let bracket_types = BracketType::parse(&text, &char_types);

        let embedding_levels = Fribidi::get_paragraph_embedding_levels_ex(
            &char_types,
            Some(&bracket_types),
            paragraph_direction
        );
        let (mut embedding_levels, _, _) = embedding_levels.unwrap();

        let res = Fribidi::reorder_line(
            FriBidiFlag::Default,
            &char_types,
            paragraph_direction,
            Some(&mut embedding_levels),
            &mut text
        );

        let res_map = res.unwrap().1;

        assert_eq!(text, gt);
        assert_eq!(res_map, gt_map);
    }
}