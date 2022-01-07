// Copyright 2021 by the authors.
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

use extendr_api::prelude::*;

pub mod tuples;

/// Create all tuples of size `n`.
/// @param n Tuple size (e.g. 3 for codons)
/// @example `codons = tuples(3)`
/// @export
#[extendr]
fn all_tuples(n: u16) -> Vec<String> {
    use tuples::all_nuc_tuples;
    all_nuc_tuples(n)
}

// Example from https://www.youtube.com/watch?v=EX7YG2pmcC8
// Talk by Claus Wilke
struct Counter {
    n: i32,
}

/// @export
#[extendr]
impl Counter {
    fn new() -> Self {
        Self { n: 0 }
    }

    fn increment(&mut self) {
        self.n += 1;
    }

    fn get(&self) -> i32 {
        self.n
    }
}

// test.demo <- function() .Call(wrap__test_demo)
// r_name is not yet working. Workaround in R/r_name.R:
/// @export
#[extendr(r_name="test.demo")]
pub fn test_demo() -> Vec<i32> {
    let _rv = R!("c(1, 2, 3)").unwrap();
    // let v : RColumn<&[f64]> = rv.as_vector().unwrap();
    // let i = v.data()[0];
    vec![1, 2, 3]
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod gcatbase; // like R package name
    impl Counter;
    fn all_tuples;
    fn test_demo;
}
