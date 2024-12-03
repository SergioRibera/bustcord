pub trait Easing {
    fn eval(&self, time: f64) -> f64;
    fn velocity(&self, time: f64) -> Option<f64> {
        let _ = time;
        None
    }
    fn finished(&self, time: f64) -> bool {
        !(0. ..1.).contains(&time)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Linear;
impl Easing for Linear {
    fn eval(&self, time: f64) -> f64 {
        time
    }
}

#[derive(Clone, Debug)]
pub struct Transition {
    pub duration: u64,

    // TODO: implement more(?
    pub easing: Linear,
}

impl Transition {
    pub fn new(duration: u64) -> Self {
        Self {
            duration,
            easing: Linear,
        }
    }
}
