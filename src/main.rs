mod vector;

const WIDTH: i32 = 256;
const HEIGHT: i32 = 256;
const COLORS: i32 = 256;

fn print_header() {
    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("{}", COLORS);
}

fn main() {
    let width_color_step = f64::from(COLORS) / f64::from(WIDTH);
    let height_color_step = f64::from(COLORS) / f64::from(HEIGHT);

    // let point = vector::Point::new(&0.0, &0.0, &0.0);

    print_header();

    for h in 0..HEIGHT {
        for w in 0..WIDTH {
            let r = h as f64 / (WIDTH - 1) as f64 * width_color_step;
            let g = w as f64 / (HEIGHT - 1) as f64 * height_color_step;
            let c = vector::Vec3::new_with_coords(r, g, 0.25);
            c.write_color();
        }
    }
}
