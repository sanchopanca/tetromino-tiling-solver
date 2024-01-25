use tetromino::{print_figure, solve, Shape, Tetromino};

fn main() {
    let n_rows = 8;
    let n_cols = 4;

    let field = vec![vec![None; n_cols]; n_rows];

    let result = solve(
        &field,
        &[
            Tetromino::new(Shape::O),
            Tetromino::new(Shape::I),
            // Tetromino::new(Shape::O),
            // Tetromino::new(Shape::I),
            Tetromino::new(Shape::T),
            Tetromino::new(Shape::T),
            // Tetromino::new(Shape::T),
            // Tetromino::new(Shape::T),
            Tetromino::new(Shape::L),
            // Tetromino::new(Shape::L),
            Tetromino::new(Shape::Z),
            // Tetromino::new(Shape::Z),
            Tetromino::new(Shape::O),
            // Tetromino::new(Shape::O),
            Tetromino::new(Shape::I),
            // Tetromino::new(Shape::I),
        ],
    );

    print_figure(&result.unwrap());
}
