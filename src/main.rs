use chrono::DateTime;
use rusqlite::{params, Connection, Result};
use serde_json::Value;
use std::process::Command;
use std::{thread, time};
use structopt::StructOpt;

static DB_FILE: &str = "./speedtest_daemon.db";

static TABLE_QUERY: &str = "
CREATE TABLE IF NOT EXISTS results (
    timestamp NUMBER,
    download NUMBER,
    upload NUMBER,
    ping NUMBER,
    bytes_sent NUMBER,
    bytes_received NUMBER
);";

static INSERT_QUERY: &str = "
INSERT INTO results (
timestamp, download, upload, ping, bytes_sent, bytes_received
)
VALUES (
?1, ?2, ?3, ?4, ?5, ?6 
);";

//Create SQL Table if not already exists for SQLite persistance.
fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(TABLE_QUERY)
        .expect("Failed to execute table query");
    Ok(())
}

#[derive(Debug)]
struct TestResult {
    timestamp: i64,
    download: f64,
    upload: f64,
    ping: f64,
    bytes_sent: f64,
    bytes_received: f64,
}

//insert speedtest result into sqlite database
fn insert_result(result: &TestResult, conn: &Connection) -> Result<()> {
    conn.execute(
        INSERT_QUERY,
        params![
            result.timestamp,
            result.download,
            result.upload,
            result.ping,
            result.bytes_sent,
            result.bytes_received
        ],
    )?;
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "SpeedTestDaemon",
    about = "Speedtest Daemon that runs test and puts data into SQL Lite Database."
)]
struct Opt {
    #[structopt(short="db", long="database", default_value=DB_FILE)]
    db: String,

    #[structopt(short = "i", long = "interval", default_value = "5000")]
    interval: u64,

    #[structopt(short, long, default_value = "-1")]
    runs: i64,
}

#[macro_use]
extern crate log;

//Entry point for the daemon, currently just runs once but at some point could schedule to run
//repeatedly in a loop or until killed.
fn main() {
    env_logger::init();
    let opt = Opt::from_args();
    let cn = Connection::open(opt.db.as_str()).expect("Could not open connection to file!");
    create_tables(&cn).expect("Could not create SQL tables!");

    let mut i: i64 = 0;

    if opt.runs != -1 {
        info!("Number of runs : {}", opt.runs);
    } else {
        info!("Daemon will run indefinitely.");
    }

    while (opt.runs == -1) || (i < opt.runs) {
        let speedtest = Command::new("speedtest-cli")
            .arg("--json")
            .output()
            .expect("Speedtest cli failed!");
        let parsed: Value =
            serde_json::from_slice(&speedtest.stdout).expect("JSON output could not be parsed!");
        let result = TestResult {
            timestamp: DateTime::parse_from_rfc3339(parsed["timestamp"].as_str().unwrap())
                .unwrap()
                .timestamp(),
            download: parsed["download"].as_f64().unwrap(),
            upload: parsed["upload"].as_f64().unwrap(),
            ping: parsed["ping"].as_f64().unwrap(),
            bytes_sent: parsed["bytes_sent"].as_f64().unwrap(),
            bytes_received: parsed["bytes_received"].as_f64().unwrap(),
        };

        insert_result(&result, &cn).expect("Insert failed!");
        i += 1;
        let duration = time::Duration::from_millis(opt.interval);
        thread::sleep(duration);
    }
    cn.close().expect("Could not close connection to database");
}
