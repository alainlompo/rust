// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![feature(rustc_attrs)]
// Test slicing expressions doesn't defeat the borrow checker.

fn main() { #![rustc_error] // rust-lang/rust#49855
    let y;
    {
        let x: &[isize] = &vec![1, 2, 3, 4, 5];
        y = &x[1..];
    }
}
