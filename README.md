# rust-fribidi
Rust binding for [fribidi](https://github.com/fribidi/fribidi) library (Unicode bidirectional and Arabic joining/shaping algorithms).

# example
```rust
let text = U32String::from("چرمهين");

let gt = U32String::from("ﻦﻴﻬﻣﺮﭼ");
let gt_maximum_level = 2;
let gt_positions_logic_to_visual = vec![5, 4, 3, 2, 1, 0];
let gt_positions_visual_to_logic = vec![5, 4, 3, 2, 1, 0];
let gt_embedding_levels = vec![LevelType(1); 6];

let mut positions_logic_to_visual :Vec<i32> = vec![1; text.len()];
let mut positions_visual_to_logic :Vec<i32> = vec![1; text.len()];
let mut embedding_levels: Vec<LevelType> = vec![LevelType(1); text.len()];

let (res, maximum_level) = Fribidi::logic_to_visual(
    &text,
    ParagraphType::OtherNeutral,    // let fribidi detect the type
    Some(&mut positions_logic_to_visual),
    Some(&mut positions_visual_to_logic),
    Some(&mut embedding_levels)
).unwrap();

assert_eq!((res, maximum_level), (gt, gt_maximum_level));
assert_eq!(positions_logic_to_visual, gt_positions_logic_to_visual);
assert_eq!(positions_visual_to_logic, gt_positions_visual_to_logic);
assert_eq!(embedding_levels, gt_embedding_levels);
```

# NOTE
- fribidi library is under LGPL license.
- this library is under MIT license.