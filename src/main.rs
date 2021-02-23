mod vector;

fn main() {
    let width = 256;
    let height = 256;
    let colors = 255;
    
    let width_color_step = colors as f32 / width as f32;
    let height_color_step = colors as f32 / height as f32;

    // let point = vector::Point::new(&0.0, &0.0, &0.0);

    println!("P3");
    println!("{} {}", width, height);
    println!("{}", colors);

    for h in 0..height {
        for w in 0..width {
            let r = h as f32 * height_color_step;
            let g = w as f32 * width_color_step;
            let b = 64;
            println!("{} {} {}", r as i32, g as i32, b);
        }
    }
}
