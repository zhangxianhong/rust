#![feature(repr_simd, platform_intrinsics)]

#[repr(simd)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
struct i32x2(i32, i32);
#[repr(simd)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
struct i32x3(i32, i32, i32);
#[repr(simd)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
struct i32x4(i32, i32, i32, i32);
#[repr(simd)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(non_camel_case_types)]
struct i32x8(i32, i32, i32, i32,
             i32, i32, i32, i32);

fn main() {
    let _x2 = i32x2(20, 21);
    let _x3 = i32x3(30, 31, 32);
    let _x4 = i32x4(40, 41, 42, 43);
    let _x8 = i32x8(80, 81, 82, 83, 84, 85, 86, 87);

    let _y2 = i32x2(120, 121);
    let _y3 = i32x3(130, 131, 132);
    let _y4 = i32x4(140, 141, 142, 143);
    let _y8 = i32x8(180, 181, 182, 183, 184, 185, 186, 187);

}
