pub fn run() {
    //Default is i32
    let _x = 1;
    // Default is f64
    let _y = 2.5;
    // Add explicit type
    let _z: i64 = 444444444;
    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    // boolean
    let is_active: bool = true;
    // Get bool from expression
    let is_gt_than_ten: bool = 10 > 5;
    // Uniciode
    let a1: char = 'a';
    let face: char = '\u{1F600}';
    println!("{:?}", (is_active, _x, _y, _z, is_gt_than_ten, a1, face));
}
