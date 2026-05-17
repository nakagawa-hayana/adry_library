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

pub trait FromInput: Sized {
    fn read_from<F: FnMut() -> String>(next: &mut F) -> Self;
}

macro_rules! __impl_from_input_via_parse {
    ($($t:ty),* $(,)?) => {
        $(
            impl FromInput for $t {
                fn read_from<F: FnMut() -> String>(next: &mut F) -> Self {
                    next().parse().expect("Parse error")
                }
            }
        )*
    };
}

__impl_from_input_via_parse!(
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    bool,
    String, char,
);

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
        <$t as $crate::io::FromInput>::read_from(&mut $next)
    };
}

#[macro_export]
macro_rules! __query_field_type {
    (usize1) => { usize };
    (chars) => { Vec<char> };
    ([ $t:tt ; $_len:expr ]) => { Vec<$crate::__query_field_type!($t)> };
    (( $($t:tt),* )) => { ( $($crate::__query_field_type!($t)),* ) };
    ($t:ty) => { $t };
}

#[macro_export]
macro_rules! define_query {
    (
        $(#[$attr:meta])*
        enum $name:ident {
            $($variant:ident { $($field:ident : $t:tt),* $(,)? }),* $(,)?
        }
    ) => {
        $(#[$attr])*
        enum $name {
            $($variant { $($field: $crate::__query_field_type!($t),)* }),*
        }
        impl $crate::io::FromInput for $name {
            #[allow(unused_assignments)]
            fn read_from<__F: ::std::ops::FnMut() -> ::std::string::String>(__next_ref: &mut __F) -> Self {
                let mut __next = move || __next_ref();
                let __disc: usize = <usize as $crate::io::FromInput>::read_from(&mut __next);
                let mut __idx: usize = 0;
                $(
                    __idx += 1;
                    if __disc == __idx {
                        $(let $field = $crate::read_value!(__next, $t);)*
                        return $name::$variant { $($field),* };
                    }
                )*
                panic!(
                    "define_query!: unknown discriminator {} for {}",
                    __disc,
                    stringify!($name)
                );
            }
        }
    };
}

pub use input;
pub use define_query;
