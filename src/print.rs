pub fn run() {
    // Print to console
    println!("Hello from the printrs file");

    // Basic Formating
    println!("{} is from {}", "John", "Mass");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "John", "Mass", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Games");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);
}