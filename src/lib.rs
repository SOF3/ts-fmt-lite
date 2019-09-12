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

use std::io::Write;

use derive_builder::Builder;
use proc_macro2::{Delimiter, Spacing, TokenStream, TokenTree};

type Result<T = ()> = std::io::Result<T>;

pub fn print<W: Write>(ts: TokenStream, config: Config, w: W) -> Result {
    let mut buf = Buffer {
        config,
        w,
        buf: String::new(),
    };
    buf.feed_stream(ts)?;
    buf.close()?;
    Ok(())
}

struct Buffer<W: Write> {
    config: Config,
    w: W,
    #[cfg(feature = "naive-wrap")]
    buf: String,
}

impl<W: Write> Buffer<W> {
    fn put(&mut self, mut s: String) -> Result {
        #[cfg(feature = "naive-wrap")]
        {
            s = self.naive_wrap(s)?;
        }
        self.w.write_all(s.as_bytes())
    }

    #[cfg(feature = "naive-wrap")]
    fn naive_wrap(&mut self, s: String) -> Result<String> {
        if let Some(width) = self.config.naive_wrap {
            if self.buf.len() + s.len() > width {
                let write = format!("{}\n", self.buf.trim());
                self.buf.clear();
                self.buf += &s;
                Ok(write)
            } else {
                self.buf += &s;
                Ok(String::new())
            }
        } else {
            Ok(s)
        }
    }

    fn feed(&mut self, tt: TokenTree) -> Result {
        match tt {
            TokenTree::Punct(punct) => {
                let s = match punct.spacing() {
                    Spacing::Joint => punct.as_char().to_string(),
                    Spacing::Alone => format!("{} ", punct.as_char()),
                };
                self.put(s)
            }
            TokenTree::Ident(ident) => self.put(format!("{} ", ident)),
            TokenTree::Literal(lit) => self.put(format!("{} ", lit)),
            TokenTree::Group(group) => {
                let (left, right) = delimiter_to_char(group.delimiter());
                self.put(left)?;
                self.feed_stream(group.stream())?;
                self.put(right)?;
                Ok(())
            }
        }
    }

    fn feed_stream(&mut self, ts: TokenStream) -> Result {
        for tt in ts {
            self.feed(tt)?;
        }
        Ok(())
    }

    fn close(&mut self) -> Result {
        #[cfg(feature = "naive-wrap")]
        {
            if self.buf.len() > 0 {
                self.w.write_all(self.buf.trim().as_bytes())?;
                self.buf.clear();
            }
        }
        Ok(())
    }
}

fn delimiter_to_char(del: Delimiter) -> (String, String) {
    let (l, r) = match del {
        Delimiter::Parenthesis => ('(', ')'),
        Delimiter::Brace => ('{', '}'),
        Delimiter::Bracket => ('[', ']'),
        Delimiter::None => return (String::new(), String::new()),
    };
    (l.to_string(), r.to_string())
}

#[derive(Default, Builder)]
#[builder(pattern = "owned")]
pub struct Config {
    #[cfg(feature = "naive-wrap")]
    #[builder(default = "Some(120)")]
    naive_wrap: Option<usize>,
}

#[cfg(test)]
mod tests;
