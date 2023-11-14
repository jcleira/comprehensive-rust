fn transpose(matrix: &[&[i32]]) -> &[&[i32]] {
    for i in 0..3 {
        for j in 0..3 {
            if i < j {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
    }

    return matrix;
}

fn pretty_print(matrix: &[&[i32]]) {
    for i in 0..3 {
        for j in 0..3 {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
}

fn main() {
    let matrix: &[&[i32]] = [&[101, 102, 103], &[201, 202, 203], &[301, 302, 303]];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(&matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
