use std::fs::File;

pub struct Image {
    pub width: i32,
    pub height: i32,
    pub data: String,
}

impl Image {
    /// Write the data in an `Image` to a file.
    pub fn print(&self, filename: &str) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");
        println!("{}", self.data)
    }

    /// Create an `Image` with a generic rainbow for testing purposes
    pub fn test(width: i32, height: i32) -> Image {
        let mut data: String = String::from("");

        for y in (0..height).rev() {
            for x in 0..width {
                let y = y as f64;
                let x = x as f64;

                let r = x / ((width - 1) as f64);
                let g = y / ((height - 1) as f64);
                let b = 0.25;

                let rr = (255.999 * r) as i32;
                let gg = (255.999 * g) as i32;
                let bb = (255.999 * b) as i32;

                let line = format!("{} {} {}\n", rr, gg, bb);

                data.push_str(&line);
            }
        }

        Image {
            width,
            height,
            data,
        }
    }
}
