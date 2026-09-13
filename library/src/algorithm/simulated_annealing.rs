use std::marker::PhantomData;

use crate::misc::rand::Pcg32;
use crate::misc::timer::get_time;

pub trait AnnealingScore: Copy {
    fn to_f64(self) -> f64;
}

macro_rules! impl_annealing_score {
    ($($t:ty),*) => {
        $(impl AnnealingScore for $t {
            #[inline]
            fn to_f64(self) -> f64 {
                self as f64
            }
        })*
    };
}

impl_annealing_score!(i32, i64, i128, isize, u32, u64, u128, usize, f32, f64);

pub trait AnnealingState<T> {
    fn pre_process(&mut self); // 遷移候補の生成など、eval 前の準備
    fn eval(&self) -> Option<T>; // evalした結果得られるスコアを返却。変更は行わない
    fn rollback(&mut self); // 元に戻す処理など(reject)
    fn post_process(&mut self, new_score: T); // accept
    fn current_score(&self) -> T;
}

/// スコア最大化の焼きなまし。温度は start_temp から end_temp へ線形に変化する
pub struct SimulatedAnnealing<S, T>
where
    S: AnnealingState<T>,
{
    pub state: S,
    start_temp: f64,
    end_temp: f64,
    _marker: PhantomData<T>, // 型Tを明示する
}

impl<S: AnnealingState<T>, T: AnnealingScore> SimulatedAnnealing<S, T> {
    pub fn new(state: S, start_temp: f64, end_temp: f64) -> Self {
        Self {
            state,
            start_temp,
            end_temp,
            _marker: PhantomData,
        }
    }

    pub fn execute(&mut self, limit: f64) {
        let mut rng = Pcg32::new();
        let start = get_time();
        while get_time() < limit {
            self.state.pre_process();
            if let Some(new_score) = self.state.eval() {
                let progress = (get_time() - start) / (limit - start);
                let temp = self.start_temp + (self.end_temp - self.start_temp) * progress;
                let diff = new_score.to_f64() - self.state.current_score().to_f64();
                let prob = f64::exp(diff / temp);
                if rng.gen_f64() < prob {
                    self.state.post_process(new_score);
                } else {
                    self.state.rollback();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::misc::rand::Pcg32;

    // -(x-42)^2 の最大化
    struct Sample {
        x: i64,
        prev: i64,
        rng: Pcg32,
    }

    impl Sample {
        fn score_of(x: i64) -> i64 {
            -(x - 42) * (x - 42)
        }
    }

    impl AnnealingState<i64> for Sample {
        fn pre_process(&mut self) {
            self.prev = self.x;
            self.x += if self.rng.gen_range(0..2u32) == 0 {
                1
            } else {
                -1
            };
        }
        fn eval(&self) -> Option<i64> {
            Some(Self::score_of(self.x))
        }
        fn rollback(&mut self) {
            self.x = self.prev;
        }
        fn post_process(&mut self, _new_score: i64) {}
        fn current_score(&self) -> i64 {
            Self::score_of(self.prev)
        }
    }

    #[test]
    fn converges_to_maximum() {
        let state = Sample {
            x: 0,
            prev: 0,
            rng: Pcg32::with_seed(1, 2),
        };
        let mut sa = SimulatedAnnealing::new(state, 1.0, 0.001);
        sa.execute(get_time() + 0.1);
        assert!((sa.state.x - 42).abs() <= 3, "x = {}", sa.state.x);
    }
}
