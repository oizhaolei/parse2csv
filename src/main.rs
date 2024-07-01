use regex::Regex;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    regex: String,

    /// Number of times to greet
    #[arg(short, long)]
    input: String,
}

fn get_all_capture_captions(text: &str) -> Vec<String> {
    let re_capture = Regex::new("\\?<([^>]+)>").unwrap();

    // Create a vector to store all captures
    let mut captures = Vec::new();

    // Iterate over all matches and collect captures
    for cap in re_capture.captures_iter(text) {
        for i in 1..cap.len() {
            if let Some(m) = cap.get(i) {
                captures.push(m.as_str().to_string());
            }
        }
    }
    captures
}

fn escape_quotes(input: &str) -> String {
    input.replace("\"", "\\\"")
}

/// usage:
/// access_log_parser --inputfile ./out/xxxxx.mail.com --regex "^(?<ip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}) - - \[(?<date>\S+ \+\d{4})\] \"(?<method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (?<path>\S+) (?<version>\S+)\" (?<code>\d{3}) (?<rt>\S+) \"(?<referer>\S+)\" \"(?<ua>[^\"]+)\" \"(\S+)\""
fn main() -> io::Result<()> {
    let args = Args::parse();
    let re = Regex::new(&args.regex).unwrap();

    let captions = get_all_capture_captions(&args.regex);

    // Open the file in read-only mode
    let input = File::open(&args.input)?;

    let output_filename = format!("{}.csv", &args.input);
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&output_filename)
        .unwrap();

    // write csv header
    if let Err(e) = writeln!(output, "\"{}\"", captions.join("\",\"")) {
        eprintln!("Couldn't write to file: {}", e);
    }

    // Create a buffered reader for the file
    let reader = BufReader::new(input);

    // Read the file line by line
    for line in reader.lines() {
        // Handle any errors that may occur while reading a line
        let line = line?;
        let Some(caps) = re.captures(&line) else {
            eprintln!("ignored: {}", &line);
            continue;
        };
        let fields = captions
            .iter()
            .map(|c| escape_quotes(&caps[c.as_str()]))
            .collect::<Vec<_>>()
            .join("\",\"");
        // write csv line
        if let Err(e) = writeln!(output, "\"{}\"", fields) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    println!("read from {}, write to {}", &args.input, &output_filename);

    Ok(())
}
