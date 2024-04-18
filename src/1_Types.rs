fn main() {
    let first_letter = 'A';
    let space: char = ' '; // A space inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face: char = '😺'; // Emojis are chars too
    let my_number = 100; // We didn't write a type of integer,
    // so Rust chooses i32. Rust always
    // chooses i32 for integers if you don't
    // tell it to use a different type
    let small_number: u8 = 10;
    let my_float: f64 = 5.0; // The compiler sees an f64
}