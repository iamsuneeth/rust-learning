struct Rectangle {
    height: f64,
    width: f64,
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.height * self.width
    }
    fn new(height: f64, width: f64) -> Rectangle {
        Rectangle { height, width }
    }
    fn scale(&mut self, value: f64) {
        self.height *= value;
        self.width *= value;
    }
}

pub fn test_shape() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}
