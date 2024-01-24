use tetramino::{print_figure, solve, Shape, Tetramino};

fn main() {
    let n_rows = 8;
    let n_cols = 4;

    let field = vec![vec![None; n_cols]; n_rows];

    let result = solve(
        &field,
        &[
            Tetramino::new(Shape::O),
            Tetramino::new(Shape::I),
            // Tetramino::new(Shape::O),
            // Tetramino::new(Shape::I),
            Tetramino::new(Shape::T),
            Tetramino::new(Shape::T),
            // Tetramino::new(Shape::T),
            // Tetramino::new(Shape::T),
            Tetramino::new(Shape::L),
            // Tetramino::new(Shape::L),
            Tetramino::new(Shape::Z),
            // Tetramino::new(Shape::Z),
            Tetramino::new(Shape::O),
            // Tetramino::new(Shape::O),
            Tetramino::new(Shape::I),
            // Tetramino::new(Shape::I),
        ],
    );

    print_figure(&result.unwrap());
}
