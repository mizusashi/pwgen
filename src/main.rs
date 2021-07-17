use rand::{prelude::ThreadRng, Rng};
fn main() {
    let mut char_set = String::new();
    char_set.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    char_set.push_str("abcdefghijklmnopqrstuvwxyz");
    char_set.push_str("0123456789");
    let pwgen = PasswordGenerator::new(char_set);
    println!("{}", pwgen.gen());
    println!("{}", pwgen.gen());
    println!("{}", pwgen.gen());
    println!("{}", pwgen.set_range(8, 16).gen());
}

struct PasswordGenerator {
    rng: ThreadRng,
    range: (u32, u32),
    char_set: String,
}

impl PasswordGenerator {
    fn new(char_set: String) -> PasswordGenerator {
        PasswordGenerator {
            rng: rand::thread_rng(),
            range: (8, 8),
            char_set: char_set.into(),
        }
    }

    fn gen(&self) -> String {
        let mut rng = self.rng.clone();
        let len = self.char_set.chars().count() as u32;
        let mut random_string = String::new();
        let pwlen = rng.gen_range(self.range.0, 1 + self.range.1);
        for _n in 1..=pwlen {
            let random_num: u32 = rng.gen_range(0, len);
            random_string.push(self.char_set.chars().nth(random_num as usize).unwrap());
        }
        random_string
    }

    fn set_range(self, low: u32, high: u32) -> PasswordGenerator {
        PasswordGenerator {
            range: if low > high { (high, low) } else { (low, high) },
            ..self
        }
    }
}
