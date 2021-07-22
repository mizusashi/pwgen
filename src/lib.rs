use rand::{prelude::ThreadRng, Rng};
pub struct PasswordGenerator {
    rng: ThreadRng,
    range: (u32, u32),
    char_set: String,
}

impl PasswordGenerator {
    pub fn new(char_set: String) -> PasswordGenerator {
        PasswordGenerator {
            rng: rand::thread_rng(),
            range: (8, 8),
            char_set: char_set.into(),
        }
    }

    pub fn gen(&self) -> String {
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

    pub fn set_range(self, low: u32, high: u32) -> PasswordGenerator {
        PasswordGenerator {
            range: if low > high { (high, low) } else { (low, high) },
            ..self
        }
    }
}
