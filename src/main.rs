fn main() {
    // println!("Hello, world!");


    // ========================================
    // Integer Types
    // ========================================

    let x: i32 = -10;
    let y: u64 = 20;

    println!("Signed: {}", x);
    println!("Unsigned: {}", y);

    // diff bet i32 (32bit) and i64 (64bit)
    // i32 : -2,147,483,648 to 2,147,483,647
    // i64 : -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    // i64 can store more positive and negative values than i32

    let e: i32 = 2147483647;
    let f: i64 = 9223372036854775807;
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", f);

    // ========================================
    // Floating point numbers
    // f32 and f64
    // f32 : -3.4e38 to 3.4e38
    // f64 : -1.7e308 to 1.7e308
    // f64 can store more positive and negative values than f32

    let pi: f64 = 3.14;
    println!("Maximum value of pi: {}", pi);

    // ========================================
    // Boolean
    let a: bool = true;
    let b: bool = false;
    println!("True: {}", a);
    println!("False: {}", b);

    // ========================================
    // Character
    let c: char = 'a';
    println!("Character: {}", c);
}
