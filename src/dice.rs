use rand::Rng;

pub fn random_below(stop: i128) -> i128 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..stop)
}

pub fn d(sides: i8) -> i8 {
    (random_below(sides as i128) + 1) as i8
}

pub fn dice(rolls: i8, sides: i8) -> i16 {
    c![d(sides) as i16, for i in 0..rolls].iter().sum()
}
