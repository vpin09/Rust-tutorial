struct Animal {
    name: String,
}

trait Dog {
    fn bark(&self); // bark() says it needs a &self and returns nothing
    fn run(&self); // run() says it needs a &self and returns nothing.
                   // So now we have to write them ourselves.
}

impl Dog for Animal {
    fn bark(&self) {
        println!("{}, stop barking!!", self.name);
    }
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };

    rover.bark();
    rover.run();
}
