pub struct Human {
    name: String,
    size: i32,
}

pub trait Mutant {
    fn create(name: String, size: i32) -> Self;
    fn mute(&self) {
        println!("It's time to mute");
    }

    fn power(&self) -> String;
}

impl Human {
    pub fn create(name: String, size: i32) -> Human {
        Human {
            name,
            size,
        }
    }

    pub fn grow(mut self) { // Trouble around here with moving Human
        Human::breath();
        self.size += 10;
    }

    fn breath() {
        println!("breathing");
    }
}

impl Mutant for Human {
    fn create(name: String, size: i32) -> Human {
        Human {
            name,
            size,
        }
    }
    fn power(&self) -> String {
        String::from("X-ray")
    }
}
