use clap::{App, Arg, ArgMatches, Values};
use colored::Colorize;
use image::GenericImageView;
use image::io::Reader as ImgReader;
use std::path::Path;
use std::result::Result;

// Extension trait for image::Rgb<u8>
trait TrueBlvck {
    fn is_blvck(&self) -> bool;
}
impl TrueBlvck for image::Rgb<u8> {
    fn is_blvck(&self) -> bool {
        let px = (self[0], self[1], self[2]);
        match px {
            (0, 0, 0) => true,
            _ => false,
        }
    }
}

const VALID_EXTS: [&'static str; 3] = ["png", "jpg", "jpeg"];

fn main() -> Result<(), &'static str> {
    // Args object
    let args: ArgMatches = App::new("Name: Blackie")
        .version("Version: 0.2.0")
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
    let matches: Values = args.values_of("files").ok_or("No files provided")?;

    // Retrieve all valid filenames
    let files: Vec<&str> = matches
        .filter_map(|x| {
            let p = Path::new(x);

            let _name = p.file_name()?.to_str()?;
            let extension = p.extension()?.to_str()?;
            match VALID_EXTS.contains(&extension) {
                true => Some(x),
                false => None,
            }
        })
        .collect();

    if files.len() == 0 {
        return Err("No valid files provided!");
    }

    for file in files.iter() {
        let myimg = match ImgReader::open(file) {
            Ok(img) => img.decode(),
            Err(_) => {
                eprintln!("Img decoding failed for file {}", file);
                continue;
            }
        };



        // Decode this fucker.
        // Srsly this is my first rust app and I'm writing this at 2am...
        let myimg = myimg
            .expect(format!("Image decoding was unsuccesful. File {}", file).as_str());
        
        
        let (width, height) = (myimg.width(), myimg.height());
        let myimg = myimg.into_rgb8();

        let mut tru_count: usize = 0;
        let mut all_count: usize = 0;
        for px in myimg.pixels() {
            tru_count += if px.is_blvck() { 1 } else { 0 };
            all_count += 1;
        }
        let percent = (tru_count as f32 / all_count as f32) * 100.0;
        println!("{} {}: {:.4}", file.green(), format!("({:3.0}px x {:3.0}px)", width, height).blue(), format!("{}%", percent).red());
    }
    Ok(())
}
