pub mod py {

    pub fn twod(a: f64, b: f64) -> f64 {
        return f64::sqrt((a.powf(2.0) + b.powf(2.0)));
    }

    pub fn threed(a: f64, b: f64, c: f64) -> f64 {
        return f64::sqrt((a.powf(2.0) + b.powf(2.0) + c.powf(2.0)));
    }
}
