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
regex="(?<syslog_time>.{15}) (?<cip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}) \"(?<proxy_ip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3})\",\"(?<login_id>[^\"]+)\",\"(?<local_time>.{24})\",\"(?<req_method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH|NA)\",\"(?<remote_host>.+)\",\"(?<s_ip>[^\"]+)\",\"(?<resp_code>[^\"]+)\",\"(?<req_size>\d+)\",\"(?<resp_size>\d+)\",\"(?<url_super_cat>[^\"]+)\",\"(?<url_cat>[^\"]+)\",\"(?<rist_score>\d+)\",\"(?<ua>[^\"]+)\",\"(?<action>[^\"]+)\",\"(?<rule_label>[^\"]+)\",\"(?<url_filter_rule_label>[^\"]+)\",\"(?<reason>[^\"]+)\",\"(?<ssl_decrypted>[^\"]+)\",\"(?<external_spr>[^\"]+)\".*"

target/debug/access_log_parser --regex "$regex" --input "$input"
