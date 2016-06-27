// Copyright 2015 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of using wavelet noise

extern crate noise;

use noise::{wavelet2, Seed, Point2};

mod debug;

fn main() {
    debug::render_png("wavelet2.png", &Seed::new(0), 1024, 1024, scaled_wavelet2);
    // debug::render_png("wavelet3.png", &Seed::new(0), 1024, 1024, scaled_wavelet3);
    // debug::render_png("wavelet4.png", &Seed::new(0), 1024, 1024, scaled_wavelet4);
    println!("\nGenerated wavelet2.png, wavelet3.png and wavelet4.png");
}

fn scaled_wavelet2(seed: &Seed, point: &Point2<f64>) -> f64 {
    wavelet2(seed, &[point[0] / 16.0, point[1] / 16.0])
}

// fn scaled_wavelet3(seed: &Seed, point: &Point2<f64>) -> f64 {
//     wavelet3(seed, &[point[0] / 16.0, point[1] / 16.0, point[0] / 32.0])
// }
//
// fn scaled_wavelet4(seed: &Seed, point: &Point2<f64>) -> f64 {
//     wavelet4(seed, &[point[0] / 16.0, point[1] / 16.0, point[0] / 32.0, point[1] / 32.0])
// }
