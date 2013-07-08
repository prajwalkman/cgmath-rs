// Copyright 2013 The Lmath Developers. For a full listing of the authors,
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

use std::num;

use color::{Channel, RGB, ToRGB};

#[path = "../num_macros.rs"]
mod num_macros;

#[deriving(Clone, Eq)]
pub struct HSV<T> { h: T, s: T, v: T }

impl<T> HSV<T> {
    pub fn new(h: T, s: T, v: T) -> HSV<T> {
        HSV { h: h, s: s, v: v }
    }
}

pub trait ToHSV<T> {
    pub fn to_hsv(&self) -> HSV<T>;
}

impl<T:Clone + Channel + Float> ToRGB<T> for HSV<T> {
    pub fn to_rgb(&self) -> RGB<T> {
        // Algorithm taken from the Wikipedia article on HSL and HSV:
        // http://en.wikipedia.org/wiki/HSL_and_HSV#From_HSV

        let chr = (*self).v * (*self).s;
        let h = (*self).h / num::cast(60);

        // the 2nd largest component
        let x = chr * (one!(T) - ((h % two!(T)) - one!(T)).abs());

        let mut color_rgb = cond! (
            (h < num::cast(1)) { RGB::new(chr.clone(), x, zero!(T)) }
            (h < num::cast(2)) { RGB::new(x, chr.clone(), zero!(T)) }
            (h < num::cast(3)) { RGB::new(zero!(T), chr.clone(), x) }
            (h < num::cast(4)) { RGB::new(zero!(T), x, chr.clone()) }
            (h < num::cast(5)) { RGB::new(x, zero!(T), chr.clone()) }
            (h < num::cast(6)) { RGB::new(chr.clone(), zero!(T), x) }
            _                  { RGB::new(zero!(T), zero!(T), zero!(T)) }
        );

        // match the value by adding the same amount to each component
        let mn = (*self).v - chr;

        color_rgb.r = color_rgb.r + mn;
        color_rgb.g = color_rgb.g + mn;
        color_rgb.b = color_rgb.b + mn;

        color_rgb
    }
}