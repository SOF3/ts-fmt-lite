// ts-fmt-lite
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the License);
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an AS IS BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use proc_macro2::TokenStream;
use quote::quote;

#[cfg(feature = "naive-wrap")]
fn print(ts: TokenStream) -> String {
    let config = crate::ConfigBuilder::default()
        .naive_wrap(Some(10))
        .build()
        .unwrap();
    let mut vec = vec![];
    crate::print(ts, config, &mut vec).unwrap();
    String::from_utf8(vec).unwrap()
}

#[cfg(feature = "naive-wrap")]
fn validate(ts: TokenStream, expect: &str) {
    let actual = print(ts);
    assert_eq!(&actual, expect)
}

#[cfg(feature = "naive-wrap")]
#[test]
pub fn ident_oper_short() {
    validate(quote!(a: b: c), "a : b : c");
}

#[cfg(feature = "naive-wrap")]
#[test]
pub fn ident_oper_long() {
    validate(quote!(a:b:c:d:), "a : b : c\n: d :");
}

#[cfg(feature = "naive-wrap")]
#[test]
pub fn ident_oper_joint() {
    validate(quote!(abc :: b ::c::d::), "abc :: b\n:: c :: d\n::");
}
