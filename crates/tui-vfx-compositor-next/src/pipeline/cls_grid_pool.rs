// <FILE>tui-vfx-compositor-next/src/pipeline/cls_grid_pool.rs</FILE> - <DESC>Thread-local OwnedGrid pool for the render-pipeline hot path</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Phase 1a perf work — pool is now wired into orc_render_pipeline; drop the cfg_attr(not(test), allow(dead_code)) scaffold</WCTX>
// <CLOG>0.1.1: PATCH — remove cfg_attr(not(test), allow(dead_code)); GridPool is consumed by render_pipeline_with_shadow as of orc_render_pipeline 12.1.1.
// 0.1.0: initial GridPool + PooledGrid RAII guard with inline unit tests</CLOG>

//! # GridPool
//!
//! Thread-local reuse pool for `OwnedGrid` buffers on the render-pipeline hot
//! path. A fresh `OwnedGrid::new(w, h)` allocates a `Vec<Cell>` sized to
//! `w * h`; at 80×24 that is ~7.6 KB per call, and the shadow path calls it
//! every frame. Pooling returns those buffers to a per-thread free-list keyed
//! on `(width, height)`, so steady-state allocation traffic on the render
//! thread drops to zero.
//!
//! The pool is intentionally thread-local rather than global: each render
//! thread gets its own buckets, no locking, and cleanup is automatic on
//! thread exit. If a consumer ever renders concurrently from multiple
//! threads, each thread warms its own pool on first use.
//!
//! ## Safety
//!
//! `PooledGrid::drop` uses `try_borrow_mut` on the thread-local `RefCell`; if
//! the pool is already borrowed (nested-access edge case), the grid is
//! dropped rather than returned. The next checkout allocates a fresh grid —
//! pooling is a soft optimisation, never a correctness requirement.

use std::cell::RefCell;
use std::collections::HashMap;
use tui_vfx_types::{Cell, Grid, OwnedGrid};

thread_local! {
    static GRID_POOL: RefCell<GridPool> = RefCell::new(GridPool::new());
}

/// Per-thread pool of reusable `OwnedGrid` buffers keyed by `(width, height)`.
pub(crate) struct GridPool {
    buckets: HashMap<(usize, usize), Vec<OwnedGrid>>,
}

impl GridPool {
    fn new() -> Self {
        Self {
            buckets: HashMap::new(),
        }
    }

    /// Borrow a grid of the requested size from the thread-local pool.
    ///
    /// Returns a cleared grid — if a pooled grid of the same dimensions is
    /// available, its cells are reset to `Cell::default()` and the backing
    /// `Vec` allocation is reused. Otherwise a fresh `OwnedGrid` is
    /// allocated. The returned `PooledGrid` returns its buffer to the pool
    /// on drop.
    pub(crate) fn checkout(width: usize, height: usize) -> PooledGrid {
        GRID_POOL.with(|p| {
            let mut pool = p.borrow_mut();
            let grid = match pool.buckets.get_mut(&(width, height)).and_then(Vec::pop) {
                Some(mut g) => {
                    for c in g.cells_mut() {
                        *c = Cell::default();
                    }
                    g
                }
                None => OwnedGrid::new(width, height),
            };
            PooledGrid { grid: Some(grid) }
        })
    }

    /// Test-only: number of grids currently pooled for `(width, height)`.
    #[cfg(test)]
    pub(crate) fn bucket_len(width: usize, height: usize) -> usize {
        GRID_POOL.with(|p| {
            p.borrow()
                .buckets
                .get(&(width, height))
                .map(Vec::len)
                .unwrap_or(0)
        })
    }

    /// Test-only: clear every bucket for test isolation.
    #[cfg(test)]
    pub(crate) fn reset_for_test() {
        GRID_POOL.with(|p| p.borrow_mut().buckets.clear());
    }
}

/// RAII guard for a pooled `OwnedGrid`. Returns the underlying grid to its
/// thread-local pool when dropped.
pub(crate) struct PooledGrid {
    grid: Option<OwnedGrid>,
}

impl PooledGrid {
    pub(crate) fn as_mut(&mut self) -> &mut OwnedGrid {
        self.grid.as_mut().expect("PooledGrid accessed after drop")
    }

    pub(crate) fn as_ref(&self) -> &OwnedGrid {
        self.grid.as_ref().expect("PooledGrid accessed after drop")
    }
}

impl Drop for PooledGrid {
    fn drop(&mut self) {
        if let Some(grid) = self.grid.take() {
            let key = (grid.width(), grid.height());
            GRID_POOL.with(|p| {
                if let Ok(mut pool) = p.try_borrow_mut() {
                    pool.buckets.entry(key).or_default().push(grid);
                }
                // If the pool is already borrowed (nested-access edge case),
                // silently drop the grid — next checkout will allocate a
                // fresh one.
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkout_returns_grid_of_requested_size() {
        GridPool::reset_for_test();
        let pooled = GridPool::checkout(10, 5);
        assert_eq!(pooled.as_ref().width(), 10);
        assert_eq!(pooled.as_ref().height(), 5);
    }

    #[test]
    fn checkout_returns_cleared_grid() {
        GridPool::reset_for_test();
        let pooled = GridPool::checkout(4, 4);
        for cell in pooled.as_ref().cells() {
            assert_eq!(*cell, Cell::default());
        }
    }

    #[test]
    fn drop_returns_grid_to_pool() {
        GridPool::reset_for_test();
        assert_eq!(GridPool::bucket_len(3, 3), 0);
        {
            let _p = GridPool::checkout(3, 3);
        }
        assert_eq!(GridPool::bucket_len(3, 3), 1);
    }

    #[test]
    fn reused_grid_is_cleared() {
        GridPool::reset_for_test();
        {
            let mut p = GridPool::checkout(2, 2);
            // Dirty every cell so reuse must clear them.
            for c in p.as_mut().cells_mut() {
                *c = Cell::new('X');
            }
        }
        // Next checkout of the same size pulls the pooled grid back.
        let p2 = GridPool::checkout(2, 2);
        for cell in p2.as_ref().cells() {
            assert_eq!(*cell, Cell::default());
        }
    }

    #[test]
    fn different_sizes_use_separate_buckets() {
        GridPool::reset_for_test();
        let p1 = GridPool::checkout(4, 4);
        let p2 = GridPool::checkout(8, 8);
        assert_eq!(GridPool::bucket_len(4, 4), 0);
        assert_eq!(GridPool::bucket_len(8, 8), 0);
        drop(p1);
        drop(p2);
        assert_eq!(GridPool::bucket_len(4, 4), 1);
        assert_eq!(GridPool::bucket_len(8, 8), 1);
    }
}

// <FILE>tui-vfx-compositor-next/src/pipeline/cls_grid_pool.rs</FILE> - <DESC>Thread-local OwnedGrid pool for the render-pipeline hot path</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
