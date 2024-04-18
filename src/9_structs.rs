struct Colour(u8, u8, u8); // Declare the same Colour tuple struct

struct SizeAndColour {
    size: u32,
    colour: Colour, // And we put it in our new named struct
}

fn main() {
    let my_colour = Colour(50, 0, 50);

    let size_and_colour = SizeAndColour {
        size: 150,
        colour: my_colour
    };
}
