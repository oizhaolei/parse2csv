use regex::Regex;
use std::fs::remove_file;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// regex for parse
    #[arg(short, long)]
    regex: String,

    /// input file, new file named `${input}.csv` will be generated
    #[arg(short, long)]
    input: String,
}

// parse regex string, get capture list
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

fn escape_field(input: &str) -> String {
    input.replace('"', r#""""#)
}

/// usage:
/// access_log_parser --input./out/xxxxx.mail.com --regex "^(?<ip>\\d{1,3}.\\d{1,3}.\\d{1,3}.\\d{1,3}) - - \[(?<date>\S+ \+\\d{4})\] \"(?<method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (?<path>\S+) (?<version>\S+)\" (?<code>\\d{3}) (?<rt>\S+) \"(?<referer>\S+)\" \"(?<ua>[^\"]+)\" \"(\S+)\""
fn main() -> io::Result<()> {
    let args = Args::parse();
    let re = Regex::new(&args.regex).unwrap();

    let columns = get_all_capture_captions(&args.regex);

    // Open the file in read-only mode
    let input = File::open(&args.input)?;

    let output_filename = format!("{}.csv", &args.input);
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&output_filename)
        .unwrap();
    let err_filename = format!("{}.err", &args.input);
    let mut errput = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&err_filename)
        .unwrap();

    // write csv header
    if let Err(e) = writeln!(output, "\"{}\"", columns.join("\",\"")) {
        eprintln!("Couldn't write to file: {}", e);
    }

    // Create a buffered reader for the file
    let reader = BufReader::new(input);

    let mut has_err = false;
    // Read the file line by line
    for line in reader.lines() {
        // Handle any errors that may occur while reading a line
        let line = line?;
        let Some(caps) = re.captures(&line) else {
            // parse error: write to err file
            has_err = true;
            if let Err(e) = writeln!(errput, "\"{}\"", &line) {
                eprintln!("Couldn't write to file: {}", e);
            }
            continue;
        };
        let csv_line = columns
            .iter()
            .map(|col| escape_field(&caps[col.as_str()]))
            .collect::<Vec<_>>()
            .join("\",\"");
        // write csv line
        if let Err(e) = writeln!(output, "\"{}\"", csv_line) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    println!("read from {}, write to {}", &args.input, &output_filename);
    if has_err {
        println!("write err to {}", &err_filename);
    } else {
        //delete err file
        remove_file(&err_filename)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{escape_field, get_all_capture_captions};
    #[test]
    fn test_escape_quotes() {
        assert_eq!(escape_field(""), "");
        assert_eq!(escape_field("\""), "\"\"");
        assert_eq!(escape_field("\"123\""), "\"\"123\"\"");
    }

    #[test]
    fn test_get_all_capture() {
        assert_eq!(get_all_capture_captions(""), Vec::<String>::new());
        assert_eq!(get_all_capture_captions("(?<syslog_time>.{15}) (?<cip>\\d{1,3}.\\d{1,3}.\\d{1,3}.\\d{1,3}) \"(?<proxy_ip>\\d{1,3}.\\d{1,3}.\\d{1,3}.\\d{1,3})\",\"(?<login_id>[^\"]+)\",\"(?<local_time>.{24})\",\"(?<req_method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH)\",\"(?<remote_host>[^\"]+)\",\"(?<s_ip>\\d{1,3}.\\d{1,3}.\\d{1,3}.\\d{1,3})\",\"(?<resp_code>\\d{3})\",\"(?<req_size>\\d+)\",\"(?<resp_size>\\d+)\",\"(?<url_super_cat>[^\"]+)\",\"(?<url_cat>[^\"]+)\",\"(?<rist_score>\\d+)\",\"(?<ua>[^\"]+)\",\"(?<action>[^\"]+)\",\"(?<rule_label>[^\"]+)\",\"(?<url_filter_rule_label>[^\"]+)\",\"(?<reason>[^\"]+)\",\"(?<ssl_decrypted>[^\"]+)\",\"(?<external_spr>[^\"]+)\".*"
        ),vec![
            "syslog_time",
            "cip",
            "proxy_ip",
            "login_id",
            "local_time",
            "req_method",
            "remote_host",
            "s_ip",
            "resp_code",
            "req_size",
            "resp_size",
            "url_super_cat",
            "url_cat",
            "rist_score",
            "ua",
            "action",
            "rule_label",
            "url_filter_rule_label",
            "reason",
            "ssl_decrypted",
            "external_spr"
        ] );
        assert_eq!(get_all_capture_captions("(?<syslog_time>.{26}) (?<cip>\\d{1,3}.\\d{1,3}.\\d{1,3}.\\d{1,3}) (?<cs_username>[^ ]+) (?<req_method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (?<remote_host>[^ ]+) (?<s_ip>[^ ]+) (?<s_port>[^ ]+) (?<resp_code>\\d{3}) (?<result_code>[^ ]+) (?<req_size>\\d+) (?<resp_size>\\d+) (?<resp_time>\\d+) (?<hierarchy_status>[^\n]+).*"
        ),vec![
            "syslog_time",
            "cip",
            "cs_username",
            "req_method",
            "remote_host",
            "s_ip",
            "s_port",
            "resp_code",
            "result_code",
            "req_size",
            "resp_size",
            "resp_time",
            "hierarchy_status",
        ] );
    }
}
