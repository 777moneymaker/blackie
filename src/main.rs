use clap::{App, Arg, ArgMatches, Values};
use image::{io::Reader as ImgReader, DynamicImage, ImageOutputFormat, ImageResult};

use customs::extensions;
use customs::pixel::{self, PixelStatus};

fn main() {
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
            todo!("No files provided needs implementation!");
        }
    };

    // Retrieve all valid filenames
    let (files, exts): (Vec<&str>, Vec<&str>) = pairs
        .filter(|&x| extensions::ends_with_custom(x))
        .map(|x| extensions::get_suffix(x))
        .unzip();

    if files.len() == 0 || exts.len() == 0 {
        panic!("No valid files provided!");
    }

    for (file, ext) in files.iter().zip(exts) {
        let myimg: ImageResult<DynamicImage> = match ImgReader::open(file) {
            Ok(img) => img.decode(),
            Err(_) => todo!("Failed image opening needs implementation"),
        };

        let format: ImageOutputFormat = match ext {
            ".jpg" | ".jpeg" => ImageOutputFormat::Jpeg(0),
            ".png" => ImageOutputFormat::Png,
            _ => {
                todo!("Invalid image format need implementation.")
            }
        };

        let mut tru_count: usize = 0;
        let mut all_count: usize = 0;

        // Decode this fucker.
        // Srsly this is my first rust app and I'm writing this at 2am...
        let myimg = myimg.expect("Image decoding was unsuccesful.").into_rgb8();
        for px in myimg.pixels()
        {
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
        println!(
            "Percent of trv black pixels for file {} = {:.2}%",
            file, percent
        );
    }
}
