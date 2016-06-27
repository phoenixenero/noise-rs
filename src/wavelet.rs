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

//! This implementation is different from the original paper by Robert L. Cook
//! and Tony DeRose.

use num::Float;

use {gradient, math, Seed};

// I have no idea what I'm doing. If it worked that means I just successfully
// written -- in code -- a very obscure procedural noise generation algorithm.
// If it didn't work, that means it'll probably stay obscure.

/// Linearly interpolate values.
fn lerp<T: Float>(a: T, b: T, x: T) -> T {
    a + x * (b - a)
}

/// Map value between `0.0` and `1.0` to a Quintic Hermite curve.
fn smoothstep<T: Float>(x: T) -> T {
    let _3: T = math::cast(3.0);
    let _2: T = math::cast(2.0);
    x * x * (_3 - _2 * x)
}

/// Calculate Quadratic B-spline basis function. `x` is from -1 to 1.
fn bspline_basis<T: Float>(x: T) -> T {
    let zero: T = math::cast(0.0);
    let one: T = math::cast(1.0);
    let two: T = math::cast(2.0);
    let half: T = math::cast(0.5);
    let three: T = math::cast(3.0);
    let u: T = three * ((x + one) * half);

    if zero < u && u < one {
        half * u * u
    } else if one <= u && u < two {
        let six: T = math::cast(6.0);
        half * (u * (six - two * u) - three)
    } else if two <= u && u < three {
        let c = three - u;
        half * c * c
    } else {
        zero
    }
}

fn falloff<T: Float>(x: T) -> T {
    let _1: T = math::cast(1.0);
    let _2: T = math::cast(2.0);
    (_1 - smoothstep(x) * _2) * (_1 - x * _2)
}

fn value2<T: Float>(seed: &Seed, point: &math::Point2<T>) -> T {
    #[inline(always)]
    fn get<T: Float>(seed: &Seed, corner: math::Point2<isize>) -> T {
        math::cast::<_, T>(seed.get2(corner)) * math::cast(1.0 / 255.0)
    }

    let floored = math::map2(*point, Float::floor);
    let near_corner = math::map2(floored, math::cast);
    let far_corner = math::add2(near_corner, math::one2());
    let near_distance = math::sub2(*point, floored);
    let weight = math::map2(math::sub2(*point, floored), smoothstep);

    // Corners
    let f00: T = get(seed, [near_corner[0], near_corner[1]]);
    let f10: T = get(seed, [far_corner[0], near_corner[1]]);
    let f01: T = get(seed, [near_corner[0], far_corner[1]]);
    let f11: T = get(seed, [far_corner[0], far_corner[1]]);

    let d0 = lerp(f00, f10, weight[0]);
    let d1 = lerp(f01, f11, weight[0]);
    let d = lerp(d0, d1, weight[1]);

    d * math::cast(2) - math::cast(1)
}

/// 2-dimensional wavelet noise
pub fn wavelet2<T: Float>(seed: &Seed, point: &math::Point2<T>) -> T {
    let downsampled = math::map2(*point, |v| v * math::cast(0.5));
    let v0 = value2(&seed, &point);
    let v1 = value2(&seed, &downsampled);
    (v0 - v1) * math::cast(0.5)
}

/* code graveyard

// let d = bspline_basis(near_distance[0]) * f00 + bspline_basis(- far_distance[0]) * f10;

// Get downsampled values.
let e00: T = get(seed, [near_corner[0] / 2, near_corner[1] / 2]);
let e10: T = get(seed, [far_corner[0] / 2, near_corner[1] / 2]);

let one: T = math::cast(1.0);

// Get inbetween values
let d10: T = (one - smoothstep(near_distance[0])) * e00 + (one - smoothstep(far_distance[0])) * e10;

// Subtract
let d: T = (one - smoothstep(near_distance[0])) * f00 + (one - smoothstep(far_distance[0])) * f10;

// // Downsampled corners
// let e00: T = get(seed, [near_corner[0] / 2, near_corner[1] / 2]);
// let e10: T = get(seed, [far_corner[0] / 2, near_corner[1] / 2]);
//
// // |       |       |
// // |   |   |
// let distance: isize = 2;
// let mut w10: T = math::cast(0.0);
//
// if near_corner[0].abs() % 2 == 0 {
//     w10 = near_distance[0] * math::cast(0.5);
// } else {
//     w10 = near_distance[0] * math::cast(0.5) + math::cast(0.5);
// }
// let d10: T = lerp(e00, e10, smoothstep(w10));

// *shrug* magic.
let c10n: T = f00 * falloff(near_distance[0]) + f10 * falloff(far_distance[0]);
let c01n: T = f00 * falloff(near_distance[1]) + f01 * falloff(far_distance[1]);

let c10f: T = f01 * falloff(near_distance[0]) + f11 * falloff(far_distance[0]);
let c01f: T = f10 * falloff(near_distance[1]) + f11 * falloff(far_distance[1]);

let c0 = c10n * c01n;
let c1 = c10f * c01f;
let c2 = c10n * c01f;
let c3 = c01n * c10f;

let d: T = (c0 + c1 + c2 + c3) * math::cast::<_, T>(0.25);
// let d: T = (c100 * c101 + c010 * c011) * math::cast::<_, T>(0.5);

d * math::cast(2.0) - math::cast(1.0)

*/
