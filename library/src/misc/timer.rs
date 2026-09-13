use std::sync::OnceLock;
use std::time::Instant;

/// 初回呼び出しからの経過秒数を返す
#[inline]
pub fn get_time() -> f64 {
    static START: OnceLock<Instant> = OnceLock::new();
    START.get_or_init(Instant::now).elapsed().as_secs_f64()
}
