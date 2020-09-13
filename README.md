# SpeedTestDaemon

## Description

This program is a small test program I wrote in Rust to run a subprocess that uses the `speedtest-cli` command line tool, and store the data in an in-memory SQL DB.

## Installation

This project requires:

1. Rust ( stable )
2. SQLite3
3. The speedtest-cli console application available in PATH.

## Parameters

```
USAGE:
    speedtest-daemon [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --database <db>           [default: ./speedtest_daemon.db]
    -i, --interval <interval>     [default: 5000]
    -r, --runs <runs>             [default: -1]
```

`database` is the existing or default file that this app uses to persist the data, `interval` represents the time between runs of the test, and `runs` simply is the number of times the application will test before closing. The default is -1 and so the application will run indefinitely.