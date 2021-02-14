mod dice;

fn main() {
    println!("Result d100: {}", dice::d(100));
    println!("Result 10d10: {}", dice::dice(10, 10));
}
