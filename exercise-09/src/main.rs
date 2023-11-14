struct Rectacle {
    width: u32,
    height: u32,
}

impl Rectacle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, inc: u32) {
        self.width += inc;
    }
}

fn main() {
    let mut rect = Rectacle {
        width: 30,
        height: 50,
    };

    println!("rect area: {}", rect.area());

    rect.inc_width(22);
    println!("rect area: {}", rect.area());
}
