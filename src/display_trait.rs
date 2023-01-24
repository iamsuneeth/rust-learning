use std::fmt::Display;

struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

impl Display for Satellite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name is {} and velocity is {}", self.name, self.velocity)
    }
}

pub fn test_display() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    println!("hubble is {}", hubble);
}
