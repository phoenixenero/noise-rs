// Copyright 2013 The noise-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
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

//! An example of using perlin noise

extern mod noise;

use std::rand::rng;
use noise::perlin::*;

static WIDTH: uint = 100;
static HEIGHT: uint = 100;

static GRADIENT: [&'static str, ..6] = [" ", "░", "▒", "▓", "█", "█"];

fn main() {
    let ctx = PerlinContext::new(&mut rng());

    for y in range(0, HEIGHT / 2) {
        for x in range(0, WIDTH) {
            let val = [x as f32 * 0.1,
                       y as f32 * 0.1 * 2.0].perlin(&ctx) * 0.5 + 0.5;
            print(GRADIENT[(val / 0.2) as int]);
        }
        println("");
    }
}