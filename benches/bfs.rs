#![feature(test)]

extern crate pathfinding;
extern crate test;

use pathfinding::{astar, bfs, dfs, dijkstra, fringe};
use test::Bencher;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pt {
    x: u16,
    y: u16,
    _fill: [u64; 32],
}
impl Pt {
    fn new(x: u16, y: u16) -> Pt {
        Pt {
            x: x,
            y: y,
            _fill: [0u64; 32],
        }
    }
}

#[inline]
fn neighbours(pt: &Pt) -> Vec<Pt> {
    let mut ret = vec![];
    if 0 < pt.x {
        ret.push(Pt::new(pt.x - 1, pt.y))
    }
    if pt.x < 64 {
        ret.push(Pt::new(pt.x + 1, pt.y))
    }
    if 0 < pt.y {
        ret.push(Pt::new(pt.x, pt.y - 1))
    }
    if pt.y < 64 {
        ret.push(Pt::new(pt.x, pt.y + 1))
    }
    ret
}

#[bench]
fn corner_to_corner_astar(b: &mut Bencher) {
    b.iter(|| {
        assert_ne!(
            astar(
                &Pt::new(0, 0),
                |n| neighbours(n).into_iter().map(|n| (n, 1)),
                |n| (64.0 - n.x as f32).hypot(64.0 - n.y as f32) as usize,
                |n| n.x == 64 && n.y == 64,
            ),
            None
        )
    })
}

#[bench]
fn corner_to_corner_bfs(b: &mut Bencher) {
    b.iter(|| {
        assert_ne!(
            bfs(
                &Pt::new(0, 0),
                |n| neighbours(n),
                |n| n.x == 64 && n.y == 64,
            ),
            None
        )
    })
}

#[bench]
fn corner_to_corner_dfs(b: &mut Bencher) {
    b.iter(|| {
        assert_ne!(
            dfs(
                Pt::new(0, 0),
                |n| neighbours(n),
                |n| n.x == 64 && n.y == 64,
            ),
            None
        )
    })
}

#[bench]
fn corner_to_corner_dijkstra(b: &mut Bencher) {
    b.iter(|| {
        assert_ne!(
            dijkstra(
                &Pt::new(0, 0),
                |n| neighbours(n).into_iter().map(|n| (n, 1)),
                |n| n.x == 64 && n.y == 64,
            ),
            None
        )
    });
}

#[bench]
fn corner_to_corner_fringe(b: &mut Bencher) {
    b.iter(|| {
        assert_ne!(
            fringe(
                &Pt::new(0, 0),
                |n| neighbours(n).into_iter().map(|n| (n, 1)),
                |n| (64.0 - n.x as f32).hypot(64.0 - n.y as f32) as usize,
                |n| n.x == 64 && n.y == 64,
            ),
            None
        )
    })
}

#[bench]
fn no_path_astar(b: &mut Bencher) {
    b.iter(|| {
        assert_eq!(
            astar(
                &Pt::new(2, 3),
                |n| neighbours(n).into_iter().map(|n| (n, 1)),
                |_| 1,
                |_| false,
            ),
            None
        )
    })
}

#[bench]
fn no_path_bfs(b: &mut Bencher) {
    b.iter(|| {
        assert_eq!(
            bfs(
                &Pt::new(2, 3),
                |n| neighbours(n),
                |_| false,
            ),
            None
        )
    });
}

#[bench]
fn no_path_dijkstra(b: &mut Bencher) {
    b.iter(|| {
        assert_eq!(
            dijkstra(
                &Pt::new(2, 3),
                |n| neighbours(n).into_iter().map(|n| (n, 1)),
                |_| false,
            ),
            None
        )
    });
}

#[bench]
fn no_path_fringe(b: &mut Bencher) {
    b.iter(|| {
        assert_eq!(
            fringe(
                &Pt::new(2, 3),
                |n| neighbours(n).into_iter().map(|n| (n, 1)),
                |_| 1,
                |_| false,
            ),
            None
        )
    })
}