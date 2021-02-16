#[macro_use(c)]
extern crate cute;
mod dice;


fn main() {
    use dice::d;
    println!("\nSingle Die: d(i8) -> i8");
    println!("d1: d(1) => {}, always 1", d(1));
    println!("d10: d(10) => {}, 1-10 flat uniform", d(10));
    println!("d100: d(100) => {}, 1-100 flat uniform", d(100));

    use dice::dice;
    println!("\nMultiple Dice: dice(i8, i8) -> i16");
    println!("1d1: dice(1, 1) => {}, always 1", dice(1, 1));
    println!("10d10: dice(10, 10) => {}, 10-100 geometric", dice(10, 10));
    println!("100d100: dice(100, 100) => {}, 100-10000 geometric", dice(100, 100));

    use dice::ability_dice;
    println!("\nAbility Dice: ability_dice(i8) -> i8");
    println!("Top 3 of 4d6: ability_dice(4) => {}, always 3-18", ability_dice(4));
    println!("Top 3 of 5d6: ability_dice(5) => {}, always 3-18", ability_dice(5));
    println!("Top 3 of 6d6: ability_dice(6) => {}, always 3-18", ability_dice(6));

    use dice::ability_check;
    println!("\nAbility Check: ability_check(i8, i8) -> bool");
    println!("ability_check(dc=10, bonus=0) => {}", ability_check(10, 0));
    println!("ability_check(dc=13, bonus=3) => {}", ability_check(13, 3));
    println!("ability_check(dc=15, bonus=5) => {}", ability_check(15, 5));

    use dice::attack_roll;
    println!("\nAttack Roll: attack_roll(i8, i8) -> bool");
    println!("attack_roll(ac=10, bonus=0) => {}", attack_roll(10, 0));
    println!("attack_roll(ac=13, bonus=3) => {}", attack_roll(13, 3));
    println!("attack_roll(ac=15, bonus=5) => {}", attack_roll(15, 5));


}
