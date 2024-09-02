@echo off

REM Function to display usage
:usage
echo Usage: %~0 --input ^<input^>
exit /b 1

REM Check if at least 2 arguments are provided
if "%~2"=="" (
    call :usage
)

REM Parse arguments
setlocal enabledelayedexpansion
set "input="
:parse_args
if "%~1"=="" goto args_parsed
if "%~1"=="--input" (
    set "input=%~2"
    shift
    shift
    goto parse_args
) else (
    call :usage
)
:args_parsed

REM Check if input is set
if "%input%"=="" (
    call :usage
)

REM Read from input file, parse with regex, and write to the csv file
set "regex=(?<syslog_time>.{26}) (?<cip>\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}) (?<cs_username>[^ ]+) (?<req_method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH|NA|PROPFIND) (?<remote_host>[^ ]+) (?<s_ip>[^ ]+) (?<s_port>[^ ]+) (?<resp_code>[^ ]+) (?<result_code>[^ ]+) (?<req_size>\d+) (?<resp_size>\d+) (?<resp_time>\d+) (?<hierarchy_status>[^\n]+).*"

REM Replace `access_log_parser` with the equivalent tool or command in Windows
access_log_parser --regex "%regex%" --input "%input%"
