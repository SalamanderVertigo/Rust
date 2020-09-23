struct Human;

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*Waving arms furiously*");
    }
}
