enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64), // latitude, longitude
}

impl Location {
    fn display(&self) {
        match self {
            Location::Unknown => println!("Location is unknown"),
            Location::Anonymous => println!("Location is anonymous"),
            Location::Known(latitude, longitude) => {
                println!(" Location is at ({latitude}, {longitude})")
            }
        };
    }
}

pub fn print_location() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
