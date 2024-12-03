use std::time::Duration;

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
    pub duration: Duration,

    // TODO: implement more(?
    pub easing: Linear,
}

impl Transition {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            easing: Linear,
        }
    }
}
