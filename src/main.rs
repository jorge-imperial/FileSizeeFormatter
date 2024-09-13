use std::env;
use std::fs::File;

#[derive(Debug)]
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}

#[derive(Debug)]
struct Sizes {

    in_bytes : FileSize,
    in_kb : FileSize,
    in_mb : FileSize,
    in_gb : FileSize,

}
/*
impl Sizes {
    fn all_units(mut self)  {
       self.in_bytes = FileSize::Bytes(self.size);
       self.in_kb = FileSize::Bytes(self.size);
       self.in_mb = FileSize::Bytes(self.size);
       self.in_gb = FileSize::Bytes(self.size);
    }
}
*/


/**
$ cargo run "24 mb"
    Sizes {
        bytes: "24000000 bytes",
        kilobytes: "24000 kilobytes",
        megabytes: "24 megabytes",
        gigabytes: "0 gigabytes"
    }

*/

fn main( ) {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You need so specify a string to convert.");
        return;
    }

    // Input string should be a number
    let input_size =  &args[1] ;

    // Split using space
    let v: Vec<&str> = input_size.split(' ').collect();


    let size : u64;
    match v[1].to_lowercase().as_str() {
        "bytes" => size = v[0].parse::<u64>().unwrap(),
        "kb" => size = v[0].parse::<u64>().unwrap() * 1000,
        "mb" => size = v[0].parse::<u64>().unwrap() * 1000*1000,
        "gb" => size = v[0].parse::<u64>().unwrap() * 1000*1000*1000,
        _ => {
            println!("Units need to be specified (bytes/kb/mb/gb)");
            return;
        }
    }

    let s : Sizes = Sizes {
        in_bytes: FileSize::Bytes(size),
        in_kb: FileSize::Kilobytes(size as f64/1_000.0),
        in_mb: FileSize::Megabytes(size as f64/1_000_000.0),
        in_gb: FileSize::Gigabytes(size as f64/1_000_000_000.0),
    };


    println!("Size in bytes is {:?}", s);

}