use image::ImageReader;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err(Box::from(
            "Error expected two arguments \n [input_path] [output_path]",
        ));
    }
    let output_path = Path::new(&args[2]);
    let input_path = Path::new(&args[1]);
    let display = output_path.display();
    let file = match File::open(input_path) {
        Err(why) => panic!("File error {} : {}", display, why),
        Ok(file) => file,
    };
    let mut output = match File::create(output_path) {
        Err(why) => panic!("File error {} : {}", display, why),
        Ok(output) => output,
    };
    let reader = BufReader::new(file);
    let mut pixel_num: u128;
    let mut red: u128;
    let mut green: u128;
    let mut blue: u128;

    for line in reader.lines() {
        red = 0;
        blue = 0;
        green = 0;
        pixel_num = 0;
        let pth: String = line?;
        let img = ImageReader::open(&pth)?.decode()?;
        let rgb_img = img.to_rgb8();
        #[allow(unused_variables)]
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let [r, g, b] = pixel.0;
            red += r as u128;
            blue += b as u128;
            green += g as u128;
            pixel_num += 1;
        }
        red /= pixel_num;
        green /= pixel_num;
        blue /= pixel_num;
        match output.write(format!("{} -> AVG RGB -> {} {} {}\n", pth, red, green, blue).as_bytes())
        {
            Err(why) => panic!("Eneable write into {} : {}", display, why),
            Ok(_) => println!(":D"),
        }
    }
    Ok(())
}
