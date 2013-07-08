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

use std::cast;

use color::{Channel, RGB, ToRGB, ToHSV, HSVA, ToHSVA};

#[path = "../num_macros.rs"]
mod num_macros;

#[deriving(Clone, Eq)]
pub struct RGBA<T> { r: T, g: T, b: T, a: T }

impl<T> RGBA<T> {
    #[inline]
    pub fn new(r: T, g: T, b: T, a: T) -> RGBA<T> {
        RGBA { r: r, g: g, b: b, a: a }
    }
}

pub trait ToRGBA<T> {
    pub fn to_rgba(&self) -> RGBA<T>;
}

impl<T:Clone + Channel, C: ToRGB<T>> ToRGBA<T> for (C, T) {
    #[inline]
    pub fn to_rgba(&self) -> RGBA<T> {
        match *self {
            (ref c, ref a) => unsafe {
                cast::transmute((c.to_rgb(), a.clone()))
            }
        }
    }
}

impl<T:Clone + Channel + Float> ToHSVA<T> for RGBA<T> {
    #[inline]
    pub fn to_hsva(&self) -> HSVA<T> {
        match unsafe {
            cast::transmute::<&RGBA<T>, &(RGB<T>, T)>(self)
        } {
            &(ref c, ref a) => unsafe {
                cast::transmute((c.to_hsv(), a.clone()))
            }
        }
    }
}