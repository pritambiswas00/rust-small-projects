extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::{ BufReader, copy };
use std::time::Instant;

fn main() {
    if args().len() != 3 {
        eprint!("Usage: 'source' 'target'");
        return;
    }

   let source_file = File::open(args().nth(1).unwrap()).unwrap();
   let mut input = BufReader::new(&source_file);
   let target_file = File::create(args().nth(2).unwrap()).unwrap();

   //Encoding

   let mut encoder = GzEncoder::new(&target_file, Compression::default());

   let start  = Instant::now();
   match copy(&mut input, &mut encoder) {
    Ok(_) => {
        // No need for encoder.finish() here, it's already included in the copy function
        print!("Source len : {:?}", source_file.metadata().unwrap().len());
        print!("Output len : {:?}", target_file.metadata().unwrap().len());
        print!("Elapsed : {:?}", start.elapsed());
    }
    Err(_) => {
        eprintln!("Error during encoding");
    }
}    
}


