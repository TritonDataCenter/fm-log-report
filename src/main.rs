//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright 2019 Joyent, Inc.
//
// This tool post-processes the aggregated, JSON-formatted FMA logs as produced
// by running:
//
// % fmdump -AVj
//
// A file containing the output of hwgrok can optionally be specified.  If
// specified, it will be used to correlate the event telemetry with HW identity
// and location information to produce a more complete report.
//
extern crate getopts;
use getopts::Options;

use std::env;
use std::process;

extern crate fm_log_report;

fn usage(progname: &str, opts: &Options) {
    let msg = format!("USAGE: {} -f <ERRLOG> [-H HWGROK]", progname);
    print!("{}", opts.usage(&msg));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let progname = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this usage message");
    opts.optopt("H", "hwgrok", "Output of hwgrok", "HWGROK");
    opts.optopt("f", "fmlog", "FM logs as JSON", "FMLOG");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    if matches.opt_present("h") {
        usage(&progname, &opts);
        process::exit(2);
    }

    let fmlog_path = match matches.opt_str("f") {
        Some(path) => path,
        None => {
            eprintln!("-f argument is required");
            usage(&progname, &opts);
            process::exit(2);
        }
    };
    let hwgrok_path = matches.opt_str("H");

    let config = fm_log_report::Config::new(fmlog_path, hwgrok_path);

    match fm_log_report::run(&config) {
        Ok(_r) => {
            process::exit(0);
        }
        Err(e) => {
            eprintln!("An error occurred: {}", e.to_string());
            process::exit(1);
        }
    }
}
