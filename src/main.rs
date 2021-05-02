use std::io::stderr;
use std::io::Write;
mod linear_algebra;

fn main() {
    let width = 256;
    let height = 256;

    print!("P3\n{} {}\n255\n", width, height);

    let blue = 63;
    for row in 0..height {
        eprint!("\rScanlines remaining: {}", height - 1 - row);
        stderr().flush().unwrap();
        for col in 0..width {
            let red = col;
            let green = width - row + 1;
            println!("{} {} {}", red, green, blue);
        }
    }
}

