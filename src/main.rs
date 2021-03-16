fn main() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let char_set = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    let len = char_set.chars().count() as u32;

    for _n in 1..30 {
        let random_num: u32 = rng.gen_range(0, len);
        print!("{}", char_set.chars().nth(random_num as usize).unwrap());
    }
}
