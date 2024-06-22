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
regex="(?<syslog_time>.{26}) (?<cip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}) (?<cs_username>[^ ]+) (?<req_method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH|NA|PROPFIND) (?<remote_host>[^ ]+) (?<s_ip>[^ ]+) (?<s_port>[^ ]+) (?<resp_code>[^ ]+) (?<result_code>[^ ]+) (?<req_size>\d+) (?<resp_size>\d+) (?<resp_time>\d+) (?<hierarchy_status>[^\n]+).*"

parse2csv --regex "$regex" --input "$input"
