use js_sys::Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Shape::I => "I",
                Shape::J => "J",
                Shape::L => "L",
                Shape::O => "O",
                Shape::S => "S",
                Shape::T => "T",
                Shape::Z => "Z",
            }
        )
    }
}

#[derive(Debug)]
pub struct Tetromino {
    figure: Vec<Vec<Option<Shape>>>,
    shape: Shape,
}

impl Tetromino {
    pub fn new(shape: Shape) -> Tetromino {
        Tetromino {
            figure: match shape {
                Shape::I => vec![
                    vec![Some(shape)],
                    vec![Some(shape)],
                    vec![Some(shape)],
                    vec![Some(shape)],
                ],
                Shape::J => vec![
                    vec![Some(shape), Some(shape), Some(shape)],
                    vec![None, None, Some(shape)],
                ],
                Shape::L => vec![
                    vec![Some(shape), Some(shape), Some(shape)],
                    vec![Some(shape), None, None],
                ],
                Shape::O => vec![
                    vec![Some(shape), Some(shape)],
                    vec![Some(shape), Some(shape)],
                ],
                Shape::S => vec![
                    vec![None, Some(shape), Some(shape)],
                    vec![Some(shape), Some(shape), None],
                ],
                Shape::T => vec![
                    vec![Some(shape), Some(shape), Some(shape)],
                    vec![None, Some(shape), None],
                ],
                Shape::Z => vec![
                    vec![Some(shape), Some(shape), None],
                    vec![None, Some(shape), Some(shape)],
                ],
            },
            shape,
        }
    }

    fn variants(&self) -> Vec<Vec<Vec<Option<Shape>>>> {
        let number_of_rotations = match self.shape {
            Shape::O => 1,
            Shape::S | Shape::Z | Shape::I => 2,
            _ => 4,
        };
        let mut rotations = Vec::with_capacity(number_of_rotations);
        rotations.push(self.figure.clone());
        let mut figure = self.figure.clone();
        for _ in 1..number_of_rotations {
            figure = rotate(&figure);
            rotations.push(figure.clone());
        }
        rotations
    }
}

impl std::fmt::Display for Tetromino {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        print_figure_to_formatter(&self.figure, f)
    }
}

fn print_figure_to_formatter(
    figure: &[Vec<Option<Shape>>],
    f: &mut impl std::fmt::Write,
) -> std::fmt::Result {
    for row in figure {
        for cell in row {
            match cell {
                Some(x) => write!(f, "{}", x)?,
                None => write!(f, " ")?,
            }
        }
        writeln!(f)?;
    }
    Ok(())
}

pub fn print_figure(figure: &[Vec<Option<Shape>>]) {
    let mut string = String::new();
    print_figure_to_formatter(figure, &mut string).unwrap();
    println!("{}", string);
}

fn rotate<T: Copy>(figure: &[Vec<Option<T>>]) -> Vec<Vec<Option<T>>> {
    let mut new_figure = vec![vec![None; figure.len()]; figure[0].len()];
    for (i, row) in figure.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell.is_some() {
                new_figure[j][figure.len() - 1 - i] = *cell;
            }
        }
    }
    new_figure
}

pub fn solve(
    field: &[Vec<Option<Shape>>],
    figures: &[Tetromino],
) -> Option<Vec<Vec<Option<Shape>>>> {
    if figures.is_empty() {
        if is_solved(field) {
            // print_figure(field);
            return Some(field.to_vec());
        } else {
            return None;
        }
    }
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            for figure in figures[0].variants() {
                if fits_in_field(field, &figure, (i, j)) {
                    let new_field = place_in_field(field, &figure, (i, j));
                    let solution = solve(&new_field, &figures[1..]);
                    if solution.is_some() {
                        return solution;
                    }
                }
            }
        }
    }
    None
}

fn fits_in_field<T>(
    field: &[Vec<Option<T>>],
    figure: &[Vec<Option<T>>],
    (start_row, start_col): (usize, usize),
) -> bool {
    for (i, figure_row) in figure.iter().enumerate() {
        for (j, figure_cell) in figure_row.iter().enumerate() {
            if figure_cell.is_some() {
                if start_row + i >= field.len() {
                    return false;
                }
                if start_col + j >= field[0].len() {
                    return false;
                }
                if field[start_row + i][start_col + j].is_some() {
                    return false;
                }
            }
        }
    }
    true
}

fn place_in_field<T: Copy>(
    field: &[Vec<Option<T>>],
    figure: &[Vec<Option<T>>],
    (start_row, start_col): (usize, usize),
) -> Vec<Vec<Option<T>>> {
    let mut field = field.to_vec();
    for (i, figure_row) in figure.iter().enumerate() {
        for (j, figure_cell) in figure_row.iter().enumerate() {
            if figure_cell.is_some() {
                field[start_row + i][start_col + j] = *figure_cell;
            }
        }
    }
    field
}

fn is_solved<T>(field: &[Vec<Option<T>>]) -> bool {
    for row in field {
        for cell in row {
            if cell.is_none() {
                return false;
            }
        }
    }
    true
}

#[allow(dead_code)]
fn remove_duplicates_and_rotations<T: Copy + PartialEq>(
    fields: Vec<Vec<Vec<Option<T>>>>,
) -> Vec<Vec<Vec<Option<T>>>> {
    let mut result = Vec::new();
    for field in fields {
        if !result.contains(&field)
            && !result.contains(&rotate(&field))
            && !result.contains(&rotate(&rotate(&field)))
            && !result.contains(&rotate(&rotate(&rotate(&field))))
        {
            result.push(field);
        }
    }
    result
}

fn convert_solution(solution: Vec<Vec<Option<Shape>>>) -> Array {
    let result = Array::new();
    for row in solution {
        let row_array = Array::new();
        for cell in row {
            row_array.push(&JsValue::from(match cell {
                Some(x) => x.to_string(),
                None => panic!("Solution wasn't found"),
            }));
        }
        result.push(&row_array);
    }
    result
}

#[wasm_bindgen]
pub fn solve_tetromino(height: u32, width: u32, shapes: Array) -> Array {
    let shapes: Vec<Shape> = shapes
        .to_vec()
        .iter()
        .map(|x| {
            match x
                .as_string()
                .expect("Wrong argument. Shapes must be strings")
                .as_str()
            {
                "I" => Shape::I,
                "J" => Shape::J,
                "L" => Shape::L,
                "O" => Shape::O,
                "S" => Shape::S,
                "T" => Shape::T,
                "Z" => Shape::Z,
                _ => panic!("Unknown shape {:?}", x),
            }
        })
        .collect();
    let field = vec![vec![None; width as usize]; height as usize];
    let tetrominos = shapes
        .iter()
        .map(|x| Tetromino::new(*x))
        .collect::<Vec<Tetromino>>();
    let result = solve(&field, &tetrominos);
    convert_solution(result.expect("Solution wasn't found"))
}
