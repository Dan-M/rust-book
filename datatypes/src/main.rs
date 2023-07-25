fn main() {
    println!("Hello, world!");

    /*
     * Integers 8, 16, 32, 64, 128 bit
     * isize (size of architecture) - use this to e.g. index collections
     *
     * Ints can be written in hex, octal, decimal and binary
     * Can use underscore to separate in decimal
     *
     * Overflows are handled by panic if not release built.
     * Release builts will just "wrap around".
     *
     * Int divisions are truncated
     */

    // signed and unsinged
    let i8_type: i8 = -127;
    let u8_type: u8 = 127;
    let u64_type: u64 = 1_000_000_000;

    // floats either float64 (default) or float32
    let float_type = 10.0;
    let float32_type: f32 = 10.0;

    // integer division result in integer
    let div = 8 / 5;
    println!("{:.5}", div);
    // need to cast to float to get a float result
    let div: f32 = 8 as f32 / 5 as f32;
    println!("{:.5}", div);

    // get rid of warnings...
    println!("{i8_type} {u8_type} {u64_type} {float_type} {float32_type}");

    // boolean as everywhere
    let true_val = true;
    let false_val: bool = false;
    println!("{true_val} {false_val}");

    /*
     * Characters use single quote
     * 4 byte unicode Characters
     */
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let cat = '😻';
    println!("{c} {z} {cat}");

    // Tuples
    let tup = (1, 5.0, 'a');
    // destructure
    let (a, b, c) = tup;
    // access tuple value by index
    let x = tup.0;
    println!("{a} {:.1} {c} {x}", b);

    /*
     * Arrays:
     * - Fixed length
     * - Single datatype
     * - on the stack
     */
    let arr: [u8; 3] = [1, 2, 3];
    println!(" array length: {}", arr.len());
    // initializing array with same value:
    let zero_array = [0; 5];
    println!("{}, {}", zero_array[3], zero_array.len());
}
