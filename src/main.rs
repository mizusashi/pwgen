use rand::Rng;
fn main() {
    let char_set = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    let pwgen = PasswordGenerator::new(char_set, 8, 16);
    println!("{}", pwgen.generator());
}

struct PasswordGenerator {
    len: u32,
    char_set: String,
}

impl PasswordGenerator {
    fn new(char_set: impl Into<String>, low: u32, high: u32) -> PasswordGenerator {
        let mut rng = rand::thread_rng();
        PasswordGenerator {
            len: rng.gen_range(low, high + 1) as u32,
            char_set: char_set.into(),
        }
    }

    fn generator(&self) -> String {
        let len = self.char_set.chars().count() as u32;
        let mut random_string = String::new();
        let mut rng = rand::thread_rng();
        for _n in 1..=self.len {
            let random_num: u32 = rng.gen_range(0, len);
            random_string.push(self.char_set.chars().nth(random_num as usize).unwrap());
        }
        random_string
    }
}
