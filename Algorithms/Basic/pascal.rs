fn generate_pascals_triangle(n: usize) -> Vec<Vec<u64>> {
    let mut triangle: Vec<Vec<u64>> = Vec::new();

    for i in 0..n {
        let mut row: Vec<u64> = vec![1; i + 1];

        if i > 1 {
            for j in 1..i {
                row[j] = triangle[i - 1][j - 1] + triangle[i - 1][j];
            }
        }

        triangle.push(row);
    }

    triangle
}

fn print_pascals_triangle(triangle: Vec<Vec<u64>>) {
    let n = triangle.len();
    let max_width = (n * 2 - 1) * 3; // Determine the maximum width of the printed triangle

    for i in 0..n {
        let row = &triangle[i];
        let row_str = row.iter()
                         .map(|&num| num.to_string())
                         .collect::<Vec<String>>()
                         .join(" "); // Join numbers with space

        // Center each row
        let padding = (max_width - row_str.len()) / 2;
        let formatted_row = format!("{:width$}", row_str, width = padding + row_str.len());

        println!("{}", formatted_row);
    }
}

fn main() {
    let n = 5; // Number of rows
    let triangle = generate_pascals_triangle(n);
    print_pascals_triangle(triangle);
}
