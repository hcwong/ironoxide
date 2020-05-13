fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // Const expressions must be typed
    const HELLO_CONST: u32 = 10;
    println!("Const must be in upper case and can only have constant expressions: {}", HELLO_CONST);

    // To use shadowing, redeclare the assignnment with the let keyword
    // Let also effectively creates a new keyword, unlike mut
    //
    // Notes on types
    // Chars are a byte - unicode
    // Tuple access via a dot, do not need to be same type, need to declare types beforehand
    // Arrays support invalid index error checking. Array access same as other languages
}
