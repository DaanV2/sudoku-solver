use crate::grid::{
    cell::Cell,
    cell_collection::CellCollection,
    constants::{GRID_HEIGHT_RANGE, GRID_WIDTH_RANGE},
    coords::Coord,
    grid::Grid,
    mark::Mark,
    row::Row,
    slice::{Slice, SLICE_EMPTY},
};

use super::solver::{SolveResult, Solver};

/** Checks rows and columns, and determines if a mark value is occupied by a certain area:
 *
 *
 * x x x | x x x | x x x
 * x x x | x   x | x x x
 *       |       | x x x
 *
 * From these three we can conclude that the right square can only be placed in the bottom row
*/

pub struct MarkOccupy {}

impl MarkOccupy {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }

    pub fn solve(grid: &mut Grid) -> SolveResult {
        let mut changed = false;

        for row in GRID_HEIGHT_RANGE.step_by(3) {
            let r1 = &Row::new(row + 0);
            let r2 = &Row::new(row + 1);
            let r3 = &Row::new(row + 2);

            changed |= solve_set(grid, r1, r2, r3);
            changed |= solve_set(grid, r2, r1, r3);
            changed |= solve_set(grid, r3, r1, r2);
        }

        for col in GRID_WIDTH_RANGE.step_by(3) {
            let c1 = &Row::new(col + 0);
            let c2 = &Row::new(col + 1);
            let c3 = &Row::new(col + 2);

            changed |= solve_set(grid, c1, c2, c3);
            changed |= solve_set(grid, c2, c1, c3);
            changed |= solve_set(grid, c3, c1, c2);
        }

        SolveResult::from_changed(changed)
    }
}

impl Solver for MarkOccupy {
    fn name(&self) -> &'static str {
        "Mark Occupy"
    }

    fn solve(&self, grid: &mut Grid) -> SolveResult {
        MarkOccupy::solve(grid)
    }
}

const MASK_SQUARE1_EXCLUDED: Slice =
    Slice::create_mask_threes(Cell::new_empty(), Cell::mask(), Cell::mask());
const MASK_SQUARE2_EXCLUDED: Slice =
    Slice::create_mask_threes(Cell::mask(), Cell::new_empty(), Cell::mask());
const MASK_SQUARE3_EXCLUDED: Slice =
    Slice::create_mask_threes(Cell::mask(), Cell::mask(), Cell::new_empty());

pub fn solve_set<T: CellCollection>(grid: &mut Grid, check: &T, other1: &T, other2: &T) -> bool {
    let s1 = Slice::from(grid, check);

    // Check if only possible in one square
    let mut changed = false;

    let possibles = s1.or_all().only_possible();

    for mark in possibles.iter_possible() {
        // Isolate only the cells that have this mark
        let marked = s1.only_possible_value(mark);
        if let Some(square) = which_square(marked) {
            // We found a square that can only be in one place
            changed |= unset_three(grid, square * 3, other1, mark);
            changed |= unset_three(grid, square * 3, other2, mark);
        }
    }

    changed
}

#[inline(always)]
fn which_square(marked: Slice) -> Option<usize> {
    // This mark is only possible in square one?
    if marked & MASK_SQUARE1_EXCLUDED == SLICE_EMPTY {
        Some(0)
        // This mark is only possible in square two?
    } else if marked & MASK_SQUARE2_EXCLUDED == SLICE_EMPTY {
        Some(1)
        // This mark is only possible in square three?
    } else if marked & MASK_SQUARE3_EXCLUDED == SLICE_EMPTY {
        Some(2)
    } else {
        None
    }
}

pub fn unset_three<T: CellCollection>(grid: &mut Grid, start: usize, set: &T, mark: Mark) -> bool {
    let mut changed = false;

    changed |= mark_off_at(grid, set.get_coord(start + 0), mark);
    changed |= mark_off_at(grid, set.get_coord(start + 1), mark);
    changed |= mark_off_at(grid, set.get_coord(start + 2), mark);

    changed
}

#[inline(always)]
fn mark_off_at(grid: &mut Grid, coord: Coord, mark: Mark) -> bool {
    let cell = grid.get_cell_at(coord);

    if !cell.is_possible(mark) {
        return false;
    }

    grid.unset_possible_at(coord, mark);
    return true;
}

pub fn only_possible_in(mark: Mark, data1: Cell, data2: Cell, data3: Cell) -> Option<usize> {
    match (
        data1.is_possible(mark),
        data2.is_possible(mark),
        data3.is_possible(mark),
    ) {
        (true, false, false) => Some(0),
        (false, true, false) => Some(1),
        (false, false, true) => Some(2),
        _ => None,
    }
}

pub fn count(mark: Mark, data1: Cell, data2: Cell, data3: Cell) -> usize {
    let mask = mark.to_data() as usize;
    let shift = mask.trailing_zeros() as usize;

    let c = (data1.get_value() as usize & mask)
        + (data2.get_value() as usize & mask)
        + (data3.get_value() as usize & mask);

    c >> shift
}