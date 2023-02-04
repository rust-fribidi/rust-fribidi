/// Define option flags that various functions use. Each mask has
/// only one bit set.
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
#[repr(u32)]
pub enum FriBidiFlag
{
    ShapeMirroring = 0x00000001,
    ReorderNsm = 0x00000002,
    ShapeArabPres = 0x00000100,
    ShapeArabLiga = 0x00000200,
    ShapeArabConsole = 0x00000400,
    RemoveBidirectional = 0x00010000,
    RemoveJoining = 0x00020000,
    RemoveSpecials = 0x00040000,
    Default = Self::ShapeMirroring as u32 | Self::ReorderNsm as u32 | Self::RemoveSpecials as u32,
    Arabic = Self::ShapeArabPres as u32 | Self::ShapeArabLiga as u32
}

impl FriBidiFlag {}