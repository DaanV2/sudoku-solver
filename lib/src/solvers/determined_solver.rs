use crate::grid::{
    cell::Cell,
    cell_collection::CellCollection,
    constants::{GRID_HEIGHT_RANGE, GRID_WIDTH_RANGE},
    grid::Grid,
    mark::Mark,
    square::Square,
};

use super::solver::{SolveResult, Solver, SolverResult};

// The solver that turns solved cells into determined cells.
// EC if only 1 possibility is left
pub struct DeterminedSolver {}

impl DeterminedSolver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_box() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Solver for DeterminedSolver {
    fn name(&self) -> &'static str {
        "Determined Solver"
    }

    fn solve(&self, grid: Grid) -> SolverResult {
        let current: &mut Grid = &mut grid.clone();
        let mut changed = false;

        //For each square
        for coord in Square::iter_square_coords() {
            let sqr = current.get_square_at(coord);
            let r = check_searchable(current, &sqr);
            changed = changed | r;
        }

        //For each row
        for row_index in GRID_HEIGHT_RANGE {
            let row = current.get_row(row_index);
            let r = check_searchable(current, &row);
            changed = changed | r;
        }

        //For each column
        for col in GRID_WIDTH_RANGE {
            let col = current.get_column(col);
            let r = check_searchable(current, &col);
            changed = changed | r;
        }

        let mut result = SolverResult::nothing(*current);

        if changed {
            result.result = SolveResult::Updated;
        }

        result
    }
}

fn check_searchable<T: CellCollection>(grid: &mut Grid, area: &T) -> bool {
    let mut result = false;

    for index in area.iter() {
        let cell = area.get_cell(index);
        if cell.is_determined() {
            continue;
        }

        result |= set_if_possible(grid, area, &cell, index);
    }

    result
}

fn set_if_possible<T: CellCollection>(
    grid: &mut Grid,
    area: &T,
    cell: &Cell,
    index: usize,
) -> bool {
    for mark in cell.iter_possible() {
        //Loop through the rest of the area to see if the mark is possible anywhere else
        if is_only_possible_at(area, mark, index) {
            let coord = area.get_coord(index);
            let value = mark.to_value();
            let new_cell = Cell::new_with_value(value);

            grid.set_cell_at(coord, new_cell);
            return true;
        }
    }

    false
}

fn is_only_possible_at<T: CellCollection>(area: &T, mark: Mark, index: usize) -> bool {
    let mark_value = mark.to_value();

    for other_index in area.iter() {
        //Skip the current cell
        if other_index == index {
            continue;
        }

        let cell = area.get_cell(other_index);

        if let Some(value) = cell.value() {
            if value == mark_value {
                return false;
            }
        } else if cell.is_possible(mark) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use crate::{
        grid::{cell::Cell, cell_collection::CellCollection, mark::Mark},
        solvers::solver::{SolveResult, Solver},
        test::util::general_tests,
    };

    #[test]
    fn test_can_solve() {
        let grid = &mut general_tests::filled_sudoku();

        let index = 64;
        let coord = grid.get_coord(index);

        let cell = grid.get_cell_at(coord);
        let mut new_cell;

        if let Some(value) = cell.value() {
            new_cell = Cell::new_empty();
            new_cell.set_possible(Mark::from_value(value));
        } else {
            panic!("Cell should be determined");
        }

        grid.set_cell_at(coord, new_cell);

        let solver = super::DeterminedSolver::new();
        let output = solver.solve(grid.clone());

        assert_eq!(output.result, SolveResult::Updated);

        let check_cell = output.grid.get_cell_at(coord);

        assert_eq!(check_cell, cell);
    }

    #[test]
    fn test_single_missing_number() {
        let mut grid = general_tests::filled_sudoku();

        general_tests::remove_number(&mut grid, 5);

        println!("{}", grid);
        let solver = super::DeterminedSolver::new();
        let result = solver.solve(grid);

        assert_eq!(result.result, SolveResult::Updated);

        //Check that all cells with value 5 are determined
        for index in result.grid.iter() {
            let cell = result.grid.get_cell(index);

            assert!(cell.is_determined(), "Cell at {} is not determined", index)
        }
    }

    #[test]
    fn test_double_missing_number() {
        let mut grid = general_tests::filled_sudoku();

        general_tests::remove_number(&mut grid, 5);
        general_tests::remove_number(&mut grid, 1);

        println!("{}", grid);

        let result = super::DeterminedSolver::new().solve(grid);

        println!("{}", result.grid);

        assert_eq!(result.result, SolveResult::Updated);

        //Check that all cells with value 5 are determined
        for index in result.grid.iter() {
            let cell = result.grid.get_cell(index);

            assert!(cell.is_determined(), "Cell at {} is not determined", index)
        }
    }

    #[test]
    fn test_only_1_possible() {
        let mut grid = general_tests::filled_sudoku();

        for i in 0..10 {
            general_tests::remove_number(&mut grid, i);
        }

        println!("{}", grid);
        let result = super::DeterminedSolver::new().solve(grid);

        println!("{}", result.grid);
        assert_eq!(result.result, SolveResult::Updated);

        //Check that all cells with value 5 are determined
        for index in result.grid.iter() {
            let cell = result.grid.get_cell(index);

            assert!(cell.is_determined(), "Cell at {} is not determined", index)
        }
    }
}
