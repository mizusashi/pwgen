use pwgen::PasswordGenerator;

fn main() {
    let mut char_set = String::new();
    char_set.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    char_set.push_str("abcdefghijklmnopqrstuvwxyz");
    char_set.push_str("0123456789");
    // char_set.push_str("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~");
    // char_set.push_str("あいうえおかきくけこさしすせそたちつてとなにぬねのはひふへほまみむめもやゆよらりるれろわゐうゑをん");
    let pwgen = PasswordGenerator::new(char_set);
    println!("{}", pwgen.gen());
    println!("{}", pwgen.gen());
    println!("{}", pwgen.gen());
    println!("{}", pwgen.set_range(8, 16).gen());
}
