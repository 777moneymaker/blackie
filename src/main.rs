use clap::{App, Arg, ArgMatches, Values};
use colored::Colorize;
use image::{io::Reader as ImgReader, DynamicImage, ImageResult};
use std::path::Path;
use std::result::Result;

// Local files
mod pixel;
pub use crate::pixel::{PixelStatus};

fn main() -> Result<(), &'static str> {
    // Args object
    let args: ArgMatches = App::new("Name: Blackie")
        .version("Version: 0.1.0")
        .author("Author: Miłosz Chodkowski <AMU Poznań 2021>")
        .about("About: App for testing the % of true black pixels.")
        .arg(
            Arg::with_name("files")
                .multiple(true)
                .takes_value(true)
                .help("jp(e)g or png files"),
        )
        .get_matches();
    // Take some files or err
    let pairs: Values = match args.values_of("files") {
        Some(f) => f,
        None => {
            return Err("No files provided!");
        }
    };

    // Retrieve all valid filenames
    let (files, exts): (Vec<&str>, Vec<&str>) = pairs
        .map(|x| {
            let p = Path::new(x);
            (
                p.to_str().unwrap(),
                p.extension().unwrap().to_str().unwrap(),
            )
        })
        .filter(|x| vec!["jpg", "jpeg", "png"].contains(&x.1))
        .unzip();

    if files.len() == 0 && exts.len() == 0 {
        return Err("No valid files provided!");
    }

    for (file, _) in files.iter().zip(exts) {
        let myimg: ImageResult<DynamicImage> = match ImgReader::open(file) {
            Ok(img) => img.decode(),
            Err(e) => {
                eprintln!("Failed to read file {}. Error: {}", file, e);
                continue;
            }
        };

        // Decode this fucker.
        // Srsly this is my first rust app and I'm writing this at 2am...
        let myimg = myimg
            .expect(format!("Image decoding was unsuccesful. File {}", file).as_str())
            .into_rgb8();

        let mut tru_count: usize = 0;
        let mut all_count: usize = 0;
        for px in myimg.pixels() {
            match pixel::is_tru_blvck(px[0], px[1], px[2]) {
                PixelStatus::BLVCK => {
                    tru_count += 1;
                    all_count += 1;
                }
                PixelStatus::OTHER => {
                    all_count += 1;
                }
            }
        }
        let percent = (tru_count as f32 / all_count as f32) * 100.0;
        println!("{}: {:.4}%", file.green(), format!("{}", percent).red());
    }
    Ok(())
}
