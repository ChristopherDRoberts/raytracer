use rand::prelude::*;

pub fn rand(min: f64, max: f64)-> f64{
    min+(max-min)*random::<f64>()
}