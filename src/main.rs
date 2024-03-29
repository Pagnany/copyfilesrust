use csv;
use rayon::prelude::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args[1].clone();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path(path)
        .expect("Cannot read file");

    let records: Vec<_> = rdr
        .records()
        .collect::<Result<_, _>>()
        .expect("Cannot read records");

    records.par_iter().enumerate().for_each(|(i, result)| {
        if let Some(source_path) = result.get(0) {
            if let Some(destination_path) = result.get(1) {
                // check if the file already exists
                if fs::metadata(destination_path).is_ok() {
                    println!(
                        "{}: {} -> {} (already exists)",
                        i, source_path, destination_path
                    );
                    return;
                }
                // copy the file
                match fs::copy(source_path, destination_path) {
                    Ok(_) => { //println!("{}: {} -> {}", i, source_path, destination_path)
                    }
                    Err(e) => {
                        println!("{}: {} -> {} ({})", i, source_path, destination_path, e)
                    }
                }
            }
        }
    });
}
