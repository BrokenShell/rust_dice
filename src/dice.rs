use rand::Rng;

fn random_below(stop: i128) -> i128 {
    assert!(0 < stop);
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..stop);
}

pub fn d(sides: i8) -> i8 {
    return (random_below(sides as i128) + 1) as i8;
}

pub fn dice(rolls: i8, sides: i8) -> i16 {
    assert!(0 < rolls);
    return c![d(sides) as i16, for i in 0..rolls].iter().sum();
}

pub fn ability_dice(power: i8) -> i8 {
    assert!(3 <= power && power <= 9);
    let mut vector = c![d(6), for i in 0..power];
    vector.sort_by(|a, b| b.cmp(&a));
    return vector.iter().take(3).sum::<i8>();
}

pub fn ability_check(dc: i8, bonus: i8) -> bool {
    return d(20) + bonus >= dc;
}

pub fn attack_roll(ac: i8, bonus: i8) -> bool {
    return d(20) + bonus >= ac;
}
