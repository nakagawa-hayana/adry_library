use std::collections::VecDeque;

pub trait BfsHandler {
    type State;
    /// 現在の状態から遷移可能な状態の一覧を返す
    fn neighbors(&mut self, state: &Self::State) -> Vec<Self::State>;
    /// 訪問済みにマーク。`from` は遷移元状態(開始点では None)。
    fn mark_visited(&mut self, state: &Self::State, from: Option<&Self::State>);
    fn is_visited(&self, state: &Self::State) -> bool;
    /// 訪問時処理。falseを返すとこの状態からの遷移を行わない
    fn on_visited(&mut self, _state: &Self::State) -> bool {
        true
    }
    /// trueを返すと探索全体を打ち切る
    fn should_stop(&self) -> bool {
        false
    }
    /// `state` の遷移元状態を返す。復元しないハンドラはデフォルトの None のままでよい。
    fn parent(&self, _state: &Self::State) -> Option<Self::State> {
        None
    }
}
/// `BfsHandler` を用いた BFS。
/// 複数の開始状態を受け取れる。単一始点の場合は `bfs(&mut handler, [start])` で呼ぶ。
///
/// 訪問済みマークはキューに追加する時点で行われる（重複追加の防止）。
pub fn bfs<H: BfsHandler>(handler: &mut H, starts: impl IntoIterator<Item = H::State>) {
    let mut que = VecDeque::new();
    for start in starts {
        if handler.is_visited(&start) {
            continue;
        }
        handler.mark_visited(&start, None);
        que.push_front(start);
    }

    while let Some(state) = que.pop_front() {
        if handler.should_stop() {
            break;
        }
        if !handler.on_visited(&state) {
            continue;
        }
        for nxt in handler.neighbors(&state) {
            if handler.is_visited(&nxt) {
                continue;
            }
            handler.mark_visited(&nxt, Some(&state));
            que.push_back(nxt);
        }
    }
}

/// `handler.parent` を辿って `target` までの経路を復元する。
/// 返り値は始点 → `target` の順。`target` の親が辿れない場合は `target` のみを含む Vec を返す。
pub fn restore_path<H>(handler: &H, target: H::State) -> Vec<H::State>
where
    H: BfsHandler,
    H::State: Clone,
{
    let mut path = vec![target.clone()];
    let mut cur = target;
    while let Some(p) = handler.parent(&cur) {
        path.push(p.clone());
        cur = p;
    }
    path.reverse();
    path
}
