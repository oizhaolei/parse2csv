use std::fs::File;
use std::io::{ self, BufReader, BufRead};
use regex::Regex;


use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    regex: String,

    /// Number of times to greet
    #[arg(short, long )]
    inputfile: String,
}

/// usage:
/// access_log_parser --inputfile ./out/xxxxx.mail.com --regex "^(?<ip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}) - - \[(\S+ \+\d{4})\] \"(?<method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (\S+) (\S+)\" (\d{3}) (\S+) \"(\S+)\" \"([^\"]+)\" \"(\S+)\""
fn main() -> io::Result<()> {
        let args = Args::parse();
    let re = Regex::new(&args.regex).unwrap();

    // Open the file in read-only mode
    let file = File::open(args.inputfile)?;

    // Create a buffered reader for the file
    let reader = BufReader::new(file);

    // Read the file line by line
    for line in reader.lines() {
        // Handle any errors that may occur while reading a line
        let line = line?;
        let Some(caps) = re.captures(&line) else {
            continue;
        };
        let filename = &caps["method"];
        println!("method: {}", filename);

        // Process the line (for example, print it)
        // println!("{}", line);
    }

    Ok(())

    // let re_filename = Regex::new(r"/").unwrap();
    // let stdin = io::stdin();
    // for line in stdin.lock().lines() {
    //     let l = line.unwrap();
    //     let Some(caps) = re.captures(&l) else {
    //         continue;
    //     };
    //     let filename = &caps["filename"];
    //     println!("filename: {}", filename);
    //     let mut file = OpenOptions::new()
    //         .write(true)
    //         .append(true)
    //         .create(true)
    //         .open(format!("out/{}",re_filename.replace_all( filename,"_")))
    //         .unwrap();

    //     if let Err(e) = writeln!(file, "{}", &l) {
    //         eprintln!("Couldn't write to file: {}", e);
    //     }
    // }
}
