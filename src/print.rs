pub fn run() {
    // Print to console
    println!("Hello again....from print.rs");
    // Basic Format
    println!("Numbers {},{},{}", 1, 2, 3);
    // Positional
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Cheshire", "Code"
    );
    // Named
    println!(
        "{name} likes to play the {instrument}",
        name = "Paul",
        instrument = "Guitar"
    );

    // Traits
    println!("Binary:{:b} Hex:{:x} Octal:{:o}", 10, 10, 10);

    // Debug Trait - tuple
    println!("{:?}", (12, true, "Paul"));

    // Basic Maths
    println!("10 + 10 = {}", 10 + 10);
}
