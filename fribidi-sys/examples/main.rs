extern crate fribidi_sys;
use fribidi_sys::fribidi_bindings;

fn main() {
    println!("Hello, world!");

    let text: Vec<u32> = "محمد".bytes().into_iter().map(|ch| ch as u32).collect();
    let mut btypes: Vec<u32> = Vec::with_capacity(4);
    unsafe { fribidi_bindings::fribidi_get_bidi_types(text.as_ptr() as *const u32, 4, btypes.as_mut_ptr()); }
}
