#![allow(dead_code)]
const PI: f64 = std::f64::consts::PI;
const INFINITY: f64 = f64::INFINITY;

fn degrees_to_radians(degrees: f64) -> f64{
    degrees * PI / 180.0
}
