use std::cell::RefCell;
use std::io::{BufRead, BufReader, Stdin, Write};

pub struct StdinSource {
    reader: BufReader<Stdin>,
    buf: Vec<u8>,
    pos: usize,
}

impl StdinSource {
    pub fn new() -> Self {
        Self {
            reader: BufReader::new(std::io::stdin()),
            buf: Vec::new(),
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> String {
        loop {
            while self.pos < self.buf.len() && self.buf[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }
            if self.pos < self.buf.len() {
                break;
            }
            self.buf.clear();
            self.pos = 0;
            let n = self
                .reader
                .read_until(b'\n', &mut self.buf)
                .expect("stdin read failed");
            if n == 0 {
                panic!("unexpected EOF on stdin");
            }
        }
        let start = self.pos;
        while self.pos < self.buf.len() && !self.buf[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        std::str::from_utf8(&self.buf[start..self.pos])
            .expect("non-utf8 token")
            .to_string()
    }
}

impl Default for StdinSource {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    pub static SOURCE: RefCell<StdinSource> = RefCell::new(StdinSource::new());
}

pub fn next_token() -> String {
    SOURCE.with(|s| s.borrow_mut().next_token())
}

pub fn flush_stdout() {
    let _ = std::io::stdout().flush();
}

#[macro_export]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut __iter = $s.split_whitespace();
        let mut __next = || __iter.next().unwrap().to_string();
        $crate::input_inner!{__next, $($r)*}
    };
    ($($r:tt)*) => {
        $crate::io::flush_stdout();
        let mut __next = || $crate::io::next_token();
        $crate::input_inner!{__next, $($r)*}
    };
}

#[macro_export]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = $crate::read_value!($next, $t);
        $crate::input_inner!{$next $($r)*}
    };
}

#[macro_export]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $($crate::read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| $crate::read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        $crate::read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => {
        $crate::read_value!($next, usize) - 1
    };
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[macro_export]
macro_rules! input_match {
    ($($disc:literal => $path:path { $($field:ident : $t:tt),* $(,)? }),+ $(,)?) => {{
        $crate::input! { __disc: usize }
        match __disc {
            $(
                $disc => {
                    $crate::input! { $($field: $t),* }
                    $path { $($field),* }
                }
            )+
            _ => panic!("input_match!: unexpected discriminator: {}", __disc),
        }
    }};
}

pub use input;
pub use input_match;
