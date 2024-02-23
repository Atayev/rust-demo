fn print_spiral(matrix: &mut [[i32; 3]; 3], top: usize, left: usize, bottom: usize, right: usize) {
    if top > bottom || left > right {
        return;
    }

    // Print top row
    for i in left..=right {
        print!("{} ", matrix[top][i]);
    }

    // Print right column
    for i in (top + 1)..=bottom {
        print!("{} ", matrix[i][right]);
    }

    if top < bottom && left < right {
        // Print bottom row
        for i in (left..right).rev() {
            print!("{} ", matrix[bottom][i]);
        }

        // Print left column
        for i in ((top + 1)..bottom).rev() {
            print!("{} ", matrix[i][left]);
        }
    }

    print_spiral(matrix, top + 1, left + 1, bottom - 1, right - 1);
}

fn main() {
    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    print_matrix(&matrix);
    print_spiral(&mut matrix, 0, 0, 2, 2);
}

fn print_matrix(matrix: &[[i32; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
}
