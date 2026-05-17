// verification-helper: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_C&lang=ja

use library::{
    graph::bfs::{bfs, BfsHandler},
    utils::{
        input::Input,
        iterlibs::{collect::CollectIter, strs::StrUtilIter},
    },
};

struct S {
    graph: Vec<Vec<usize>>,
    dist: Vec<Option<i32>>,
}
impl BfsHandler for S {
    type State = (usize, i32);

    fn neighbors(&mut self, state: &Self::State) -> Vec<Self::State> {
        let (u, d) = *state;
        self.graph[u].iter().map(|v| (*v, d + 1)).collect_vec()
    }

    fn mark_visited(&mut self, state: &Self::State, _from: Option<&Self::State>) {
        self.dist[state.0] = Some(state.1);
    }

    fn is_visited(&self, state: &Self::State) -> bool {
        self.dist[state.0].is_some()
    }
}

fn solve(g: &Vec<Vec<usize>>) {
    let mut s = S {
        graph: g.clone(),
        dist: vec![None; g.len()],
    };

    bfs(&mut s, [(0, 0)]);
    println!(
        "{}",
        s.dist
            .iter()
            .map(|i| if let Some(val) = i { *val } else { -1 })
            .enumerate()
            .map(|(i, val)| format!("{} {val}", i + 1))
            .join("\n")
    );
}

fn main() {
    let mut ip = Input::new();
    let n = ip.next();

    let g = (0..n).fold(vec![vec![]; n], |mut g, _| {
        let u = ip.next::<usize>() - 1;
        let k = ip.next();
        g[u] = ip
            .vector::<usize>(k)
            .into_iter()
            .map(|i| i - 1)
            .collect_vec();
        g
    });
    solve(&g);
}
