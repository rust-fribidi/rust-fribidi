use fribidi_sys::fribidi_bindings;

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
    /// Weaken type for paragraph fallback purposes:
    /// LTR->WLTR, RTL->WRTL.
    pub fn weak(ch: Paragraph) -> bool
    {
        Self::WeakRightToLeft as u32 | (ch & fribidi_bindings::FRIBIDI_MASK_RTL) != 0
    }
}