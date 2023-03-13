use num::complex::Complex;

fn main() {
    let set = calc_mandelbrot_set(
        1000,
        -2.0,
        1.0,
        -1.0,
        1.0,
        100,
        24,
    );

    render(set);
}

fn calc_mandelbrot_set(
    max_iterations: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    let mut rows: Vec<Vec<usize>> = Vec::with_capacity(width);

    for y in 0..height {
        let mut row: Vec<usize> = Vec::with_capacity(height);

        for x in 0..width {
            let x_percentage = (x as f64 / width as f64);
            let y_percentage = (y as f64 / height as f64);

            let cx = x_min + (x_max - x_min) * x_percentage;
            let cy = y_min + (y_max - y_min) * y_percentage;

            row.push(val_at_pixel(cx, cy, max_iterations))
        }

        rows.push(row);
    }

    rows
}

fn val_at_pixel(
    x: f64, y: f64, max_iterations: usize,
) -> usize {
    // Complex at the origin
    let mut z = Complex::new(0.0, 0.0);

    // Complex at pixel coordinates
    let c = Complex::new(x, y);

    for i in 0..=max_iterations {
        if z.norm() > 2.0 {
            return i;
        }

        z = z * z + c
    }

    max_iterations
}

fn render(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };

            line.push(val);
        }
        println!("{}", line);
    }
}