use std::{
    ops::{Bound, RangeBounds},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::utils::integer::Integer;

static MUL: u64 = 6_364_136_223_846_793_005;

pub trait PcgRandomInt: Integer {
    fn get_random(rng: &mut Pcg32) -> Self;
    fn wrapping_neg(self) -> Self;
}

macro_rules! impl_pcg_int {
    ($t:ty, |$rng:ident| $get_random_logic:expr) => {
        impl PcgRandomInt for $t {
            #[inline]
            fn get_random($rng: &mut Pcg32) -> Self {
                $get_random_logic
            }
            #[inline]
            fn wrapping_neg(self) -> Self {
                self.wrapping_neg()
            }
        }
    };
}

impl_pcg_int!(u32, |rng| rng.next_u32());
impl_pcg_int!(u64, |rng| rng.next_u64());
impl_pcg_int!(u128, |rng| rng.next_u128());
impl_pcg_int!(usize, |rng| rng.next_u64() as usize);

impl_pcg_int!(i32, |rng| i32::from_ne_bytes(rng.next_u32().to_ne_bytes()));
impl_pcg_int!(i64, |rng| i64::from_ne_bytes(rng.next_u64().to_ne_bytes()));
impl_pcg_int!(i128, |rng| i128::from_ne_bytes(
    rng.next_u128().to_ne_bytes()
));

pub struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let stream_id = &seed as *const u64 as u64;
        Self::with_seed(seed, stream_id)
    }

    pub fn with_seed(seed: u64, stream_id: u64) -> Self {
        let inc = (stream_id << 1) | 1;
        let state = seed.wrapping_add(inc);
        let mut pcg = Self { state, inc };
        pcg.next();
        pcg
    }

    fn next(&mut self) -> u32 {
        let old_state = self.state;
        self.state = self.state.wrapping_mul(MUL).wrapping_add(self.inc);

        let xorshifted = ((old_state >> 18) ^ (old_state >> 27)) as u32;
        let rot = (old_state >> 59) as u32;
        xorshifted.rotate_right(rot)
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let u = self.next_u32() as u64;
        let d = self.next_u32() as u64;
        (u << 32) | d
    }

    #[inline]
    fn next_u128(&mut self) -> u128 {
        let u = self.next_u64() as u128;
        let d = self.next_u64() as u128;
        (u << 64) | d
    }

    /// [0, 1) の一様乱数
    #[inline]
    pub fn gen_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    pub fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: PcgRandomInt,
        R: RangeBounds<T>,
    {
        let l = match range.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + T::from_i32(1),
            Bound::Unbounded => panic!("A lower bound is needed"),
        };
        let r = match range.end_bound() {
            Bound::Included(&e) => e + T::from_i32(1),
            Bound::Excluded(&e) => e,
            Bound::Unbounded => panic!("A upper bound is needed"),
        };
        let bound = r - l;
        let threshold = T::wrapping_neg(bound) % bound;
        loop {
            let r = T::get_random(self);
            if r >= threshold {
                return l + (r % bound);
            }
        }
    }
}
