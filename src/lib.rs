//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright 2019 Joyent, Inc.
//
extern crate chrono;
use chrono::prelude::*;

extern crate serde;
use serde::Deserialize;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

mod hwgrok;
use hwgrok::HwGrok;

#[derive(Debug)]
pub struct Config {
    pub fmlog_path: String,
    pub hwgrok_path: Option<String>,
}

impl Config {
    pub fn new(fmlog_path: String, hwgrok_path: Option<String>) -> Config {
        Config { fmlog_path, hwgrok_path }
    }
}

#[derive(Debug, Deserialize)]
struct FmEvent {
    class: String,
}

#[derive(Debug, Deserialize)]
pub struct Ereport {
    class: String,
    detector: Detector,
    #[serde(rename = "__tod")]
    tod: Vec<i64>,
}

#[derive(Debug, Deserialize)]
struct Detector {
    scheme: String,

    // fields specific to dev-scheme detectors
    #[serde(rename = "device-path")]
    device_path: Option<String>,

    // fields specific to hc-scheme detectors
    #[serde(rename = "hc-list")]
    hc_list: Option<Vec<HcPair>>,

    // fields specific to fmd-scheme detectors
    #[serde(rename = "mod-name")]
    mod_name: Option<String>,
}

impl Detector {
    pub fn get_fmristr(&mut self) -> Result<String, Box<dyn Error>> {
        match self.scheme.as_ref() {
            "dev" => {
                Ok(format!("dev://{}", self.device_path.as_mut().unwrap()))
            }
            "hc" => {
                let mut fmristr = String::from("hc://");
                for hcpair in self.hc_list.as_mut().unwrap() {
                    fmristr.push_str(format!("/{}={}", hcpair.hc_name,
                        hcpair.hc_id).as_ref());
                }
                Ok(fmristr)
            }
            "fmd" => {
                Ok(format!("fmd:///mpdule/{}", self.mod_name.as_mut().unwrap()))
            }
            _ => {
                // XXX - change to return error
                Ok(String::from("unknown"))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct HcPair {
    #[serde(rename = "hc-name")]
    hc_name: String,
    #[serde(rename = "hc-id")]
    hc_id: String,
}

#[derive(Debug)]
pub struct DeviceHashEnt {
    ereport_class_hash: HashMap<String, u32>,
    ereport_ts_hash: HashMap<String, u32>,
    ereports: Vec<Ereport>,
    ereports_ts: Vec<String>,
}

impl DeviceHashEnt {
    pub fn new(ereport: Ereport, ts: String) -> DeviceHashEnt {
        let mut ereport_class_hash = HashMap::new();
        ereport_class_hash.insert(ereport.class.clone(), 1);

        let mut ereport_ts_hash = HashMap::new();
        ereport_ts_hash.insert(ts.clone(), 1);

        let ereports = vec![ereport];
        let ereports_ts = vec![ts.clone()];

        DeviceHashEnt {
            ereport_class_hash,
            ereport_ts_hash,
            ereports,
            ereports_ts,
        }
    }
}

fn get_event_timestamp(ev_tod_secs: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(ev_tod_secs, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    datetime.format("%Y-%m-%d").to_string()
}

//
// The Device Hash is a HashMap of DevHashEnt structs, hashed by a string that
// uniquely indentifies the ereport detector.  For I/O ereports, we use the
// device path as the hash key.  For other hardware ereports, we use string
// representation of the HC-scheme FMRI. The DevHashEnt struct itself contains
// a vector of ereports associated with that device path and a hash table of
// ereport counts hashed by the ereport class name as well as a hash table of
// ereport counts hashed by the day - using a string timestamp of the form
// <YYYY>-<MM>-<DD>
//
// XXX - should this be a method on DevHashEnt?
// 
fn process_event(
    device_hash: &mut HashMap<String, DeviceHashEnt>,
    key: &str,
    ereport: Ereport
) -> Result<(), Box<dyn Error>> {

    let ts = get_event_timestamp(ereport.tod[0]);
    let mut new_ts = false;

    match device_hash.entry(key.to_string()) {
        Entry::Vacant(entry) => {
            entry.insert(DeviceHashEnt::new(ereport, ts));
        }
        Entry::Occupied(mut entry) => {
            match entry.get_mut().ereport_class_hash.entry(ereport.class.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += 1;
                }
            }
            match entry.get_mut().ereport_ts_hash.entry(ts.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(1);
                    new_ts = true;
                }
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += 1;
                }
            }
            entry.get_mut().ereports.push(ereport);
            if new_ts {
                entry.get_mut().ereports_ts.push(ts);
            }
        }
    }
    Ok(())
}

//
// Open and read in the file containing hwgrok output.  Then deserialize it
// into an HwGrok struct and return that struct.
//
// XXX - should this be moved to a new() method on HwGrok?
//
fn process_hwgrok_data(hwgrok_path: &str) -> Result<HwGrok, Box<dyn Error>> {

    let hwgrok_contents = fs::read_to_string(&hwgrok_path)?;
    let hwgrok : HwGrok = serde_json::from_str(&hwgrok_contents)?;
    
    Ok(hwgrok)
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    
    let hwgrok : HwGrok = match &config.hwgrok_path {        
        Some(path) => {
            process_hwgrok_data(&path)?
        }
        None => { HwGrok::default() }
    };

    let fmlogs = fs::File::open(&config.fmlog_path)?;
    let reader = BufReader::new(fmlogs);

    let mut device_hash = HashMap::new();

    for l in reader.lines() {
        let line = l.unwrap();

        let event: FmEvent = serde_json::from_str(&line)?;

        // For now we only have code to handle ereport events.
        if !event.class.starts_with("ereport.") {
            continue;
        }
        //
        // For now we skip ZFS ereports, as serdes has problems dealing with
        // some of the numeric fields.
        //
        if event.class.starts_with("ereport.fs.") {
            continue;
        }
        //
        // We also skip fmd ereports related to log errors as their payload
        // doesn't contain a detector member.
        //
        if event.class.starts_with("ereport.fm.fmd.log_") {
            continue;
        }

        let mut ereport: Ereport = serde_json::from_str(&line)?;

        match ereport.detector.scheme.as_str() {
            "dev" => {
                let dp = ereport.detector.device_path.as_mut().unwrap().clone();
                process_event(&mut device_hash, &dp, ereport)?;
            }
            "hc" | "fmd" => {
                match &ereport.detector.get_fmristr() {
                    Ok(fmri) => {
                        process_event(&mut device_hash, &fmri, ereport)?;
                    }
                    Err(_) => {
                        eprintln!("failed to get fmri");
                    }
                }
            }
            _ => {
                eprintln!("unsupported detector scheme - skipping");
            }
        }
    }

    // Iterate through the device hash and generate a simple report
    println!();
    for (devpath, ref devent) in device_hash.iter() {
        println!("{}", "=".repeat(75));
        println!("{0: <40} {1}", "Device Path:", devpath);
        if devpath.starts_with("/pci") && devpath.contains("disk") {
            //
            // If we can find a disk matching this device path in the hwgrok
            // data then augment the report with that information.
            //
            for drive_bay in &hwgrok.drive_bays {
                match &drive_bay.disk {
                    Some(disk) => {
                        if disk.disk_device_path == devpath.to_string() {
                            println!("{0: <40} {1}", "Disk Location:",
                                drive_bay.label);
                            println!("{0: <40} {1}", "Disk Manufacturer:",
                                disk.manufacturer);
                            println!("{0: <40} {1}", "Disk Model:",
                                disk.model);
                            println!("{0: <40} {1}", "Disk Serial:",
                                disk.serial_number);
                            println!("{0: <40} {1}", "Firmware Rev:",
                                disk.disk_firmware_rev);
                            continue;
                        }
                    }
                    None => ()
                }
            }
        } else if devpath.starts_with("/pci") {
            //
            // If we can find a PCIE device matching this device path in the
            // hwgrok data then augment the report with that information.
            //
            for pci_dev in &hwgrok.pci_devices {
                if devpath.to_string() == pci_dev.pci_device_path {
                    println!("{0: <40} {1}", "Vendor Name:",
                        pci_dev.pci_vendor_name);
                    println!("{0: <40} {1}", "Device Name:",
                        pci_dev.pci_device_name);
                    println!("{0: <40} {1}", "Subsystem Name:",
                        pci_dev.pci_subsystem_name);
                    continue;
                }
            }
        }
        println!("{0: <40} {1}\n", "Total ereports:", devent.ereports.len());
        println!("{0: <40} {1}", "class", "# occurences");
        println!("{0: <40} {1}", "-----", "------------");
        for (ereport_class, ref erptent) in devent.ereport_class_hash.iter() {
            println!("{0: <40} {1}", ereport_class, erptent);
        }
        println!("\nEvent Occurrence Distribution");
        println!("-----------------------------");
        for idx in 0..devent.ereports_ts.len() {
            let ent = devent.ereport_ts_hash.get(&devent.ereports_ts[idx]);
            println!("{0: <40} {1}", devent.ereports_ts[idx], ent.unwrap());
        }
        println!();
    }

    Ok(())
}
