extern crate zip;
use std::fs::File;
use std::io::copy;

fn main() {
    std::process::exit(run_extractor());
}

fn run_extractor() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage : {} <filename> ", args[0]);
        return 1;
    }

    let file_path = std::path::Path::new(&args[1]);
    let zip_file = File::open(&file_path);
    match zip_file {
        Ok(file) => {
            match zip::ZipArchive::new(file) {
                Ok(original_data) => {
                    let mut archive_data = original_data;
                    if archive_data.len() == 0 {
                        println!("No data found.");
                        return 1;
                    } else {
                        for i in 0..archive_data.len() {
                            let mut file_data = archive_data.by_index(i).unwrap();
                            let out_path = match file_data.enclosed_name() {
                                Some(path) => {
                                    // Construct the output path relative to the current directory
                                    std::path::PathBuf::from(&path)
                                }
                                None => continue,
                            };

                            // Create the output file in the current directory
                            let mut output_file = File::create(&out_path).unwrap();
                            // Copy the contents of the file in the zip archive to the output file
                            copy(&mut file_data, &mut output_file).unwrap();
                        }
                        // Return 0 if everything is successful
                        return 0;
                    }
                }
                Err(error) => {
                    println!("Error while unzipping.. {}", error);
                    return 1;
                }
            }
        }
        Err(error) => {
            println!("Error occurred while getting file. {}", error);
            return 1;
        }
    }
}
