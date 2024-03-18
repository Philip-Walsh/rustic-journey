use std::io;

fn main() {
//     Scalar Types:

//      Integer types:
//      |   length  |   signed  |   unsigned    |
//      |   8-bit   |   i8      |   u8          |
//      |   16-bit  |   i16     |   u16         |
//      |   32-bit  |   i32     |   u32         |
//      |   64-bit  |   i64     |   u64         |
//      |   128-bit |   i128    |   u128        |
//      |   arch    |   isize   |   usize       |

//  The 32-bit unsigned integer type.
    let guess: u32 = "42".parse().expect("Not a number!");

//      Integer literals:
//      |   Number literals |   Example     |
//      |   Decimal         |   98_222      |
//      |   Hex             |   0xff        |
//      |   Octal           |   0o77        |
//      |   Binary          |   0b1111_0000 |
//      |   Byte (u8 only)  |   b'A'        |

// ===========================================
//  Floating-Point Types:
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

// ===========================================
//  Numeric Operations:
//   addition
    let sum = 5 + 10;

//   subtraction
    let difference = 95.5 - 4.3;

//   multiplication
    let product = 4 * 30;

//   division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

//   remainder
    let remainder = 43 % 5;

// ===========================================
//  The Boolean Type:
    let t = true;

    let f: bool = false; // with explicit type annotation

// ===========================================
//  The Character Type:
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

// ===========================================
//  Compound Types:
//      The Tuple Type:
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

// ===========================================
//  The Array Type:

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    //              initialize an array to contain the same value for each element
    let a = [3; 5];

//          Accessing Array Elements
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index. 1-> 5, 6+ casues error");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
