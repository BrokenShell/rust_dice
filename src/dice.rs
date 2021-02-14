use rand::Rng;

pub fn d(sides: i8) -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..sides)
}

pub fn dice(rolls: i8, sides: i8) -> i16 {
    let mut total: i16 = 0;
    for _ in 0..rolls {
        total += d(sides) as i16;
    }
    total
}
