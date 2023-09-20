#[cfg(test)]
mod test {
    use crate::{
        grid::{coords::Coord, mark::Mark, possibility::Possibility, utility::utility},
        solvers::solver_manager::SolverManager,
    };

    #[test]
    fn test_possible_when_only_1_item_is_set() {
        let grid = utility::parse_from_ascii(
            ". . . | . . . | . . .\n\
             . . . | . . . | . . .\n\
             . . . | . . . | . . .\n\
             ------|-------|------\n\
             . . . | . . . | . . .\n\
             . 9 . | . . . | . . .\n\
             . . . | . . . | . . .\n\
             ------|-------|------\n\
             . . . | . . . | . . .\n\
             . . . | . . . | . . .\n\
             . . . | . . . | . . .",
        );

        let solver = SolverManager::new();
        let result = solver.solve(grid).grid;

        let mut pos = Possibility::new();
        pos.unset_possible(Mark::N9);

        // Col 2 should have no possible 9
        for r in 0..9 {
            let c = Coord::new(r, 1);
            let cell = result.get_cell_at(c);
            if cell.is_determined() {
                continue;
            }

            assert_eq!(
                cell.possibilities, pos,
                "Cell at {} should have no possible 9",
                c
            );
        }

        // Row 4 should have no possible 9
        for c in 0..9 {
            let c = Coord::new(4, c);
            let cell = result.get_cell_at(c);
            if cell.is_determined() {
                continue;
            }

            assert_eq!(
                cell.possibilities, pos,
                "Cell at {} should have no possible 9",
                c
            );
        }

        // Square at (row 1, col 0) should have no possible 9
        for r in 0..3 {
            for c in 0..3 {
                let c = Coord::new(r + 3, c);
                let cell = result.get_cell_at(c);
                if cell.is_determined() {
                    continue;
                }

                assert_eq!(
                    cell.possibilities, pos,
                    "Cell at {} should have no possible 9",
                    c
                );
            }
        }
    }
}