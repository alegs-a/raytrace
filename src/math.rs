use rand::Rng;

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_f64_wide(lower: f64, upper: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(lower..upper)
}
