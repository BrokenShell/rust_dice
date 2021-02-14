#[macro_use(c)]
extern crate cute;
mod dice;


fn main() {
    // Single Die
    println!("\nResult d1: {}, always 1", dice::d(1));
    println!("Result d10: {}, 1-10 flat uniform", dice::d(10));
    println!("Result d100: {}, 1-100 flat uniform", dice::d(100));

    // Sum multiple rolls of the same size dice
    println!("\nResult 1d1: {}, always 1", dice::dice(1, 1));
    println!("Result 10d10: {}, 10-100 geometric", dice::dice(10, 10));
    println!("Result 100d100: {}, 100-10000 geometric", dice::dice(100, 100));
}
