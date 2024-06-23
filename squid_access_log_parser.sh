#!/usr/bin/env sh

# Function to display usage
usage() {
    echo "Usage: $0 --input <input>"
    exit 1
}

# Check if at least 2 arguments are provided
if [ "$#" -lt 2 ]; then
    usage
fi

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --input)
            input="$2"
            shift 2
            ;;
        *)
            usage
            ;;
    esac
done

# Check if input is set
if [ -z "$input" ]; then
    usage
fi

# Read from input file, parse with regex, and write to the csv file
regex="^(?<ip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}) - - \[(?<date>\S+ \+\d{4})\] \"(?<method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH) (?<path>\S+) (?<version>\S+)\" (?<code>\d{3}) (?<rt>\S+) \"(?<referer>\S+)\" \"(?<ua>[^\"]+)\" \"(\S+)\""

target/debug/access_log_parser --regex "$regex" --input "$input"
