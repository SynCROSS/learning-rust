#[warn(unused_variable)]

fn main() {
    // * Integer Type

    // * Length     Signed      Unsigned
    // * -------------------------------
    // * 8-bit      i8          u8
    // * 16-bit     i16         u16
    // * 32-bit     i32         u32
    // * 64-bit     i64         u64
    // * 128-bit    i128        u128
    // * arch       isize       usize

    // * Number literals    Example
    // * -------------------------------
    // * Decimal            98_222
    // * Hex                0xff
    // * Octal              0o77
    // * Binary             0b1111_0000
    // * Byte (u8 only)     b'A'

    // * Float Type

    let x = 2.0; // * f64, Default Float Type

    let y: f32 = 3.0; // * f32, The Other Float Type

    // * Boolean Type

    let t = true;

    let f: bool = false; // * Declaration With Explicit Type Annotation

    // * Character Type

    let c = 'z';

    let z = 'ℤ';

    let heart_eyed_cat = '😻';

    // *  Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1); // Tuple Can Contain Various Types
}
