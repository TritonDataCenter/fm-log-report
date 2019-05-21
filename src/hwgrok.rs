//
// Copyright 2019 Joyent, Inc.
//
extern crate serde;
use serde::Deserialize;

//
// The following structures are used to hold a partial deserialization of the
// JSON output from hwgrok (https://github.com/joyent/hwgrok)
//
#[derive(Debug, Default, Deserialize)]
pub struct HwGrok {
    #[serde(rename = "pci-devices")]
    pub pci_devices: Vec<HwGrokPciDevices>,
    #[serde(rename = "drive-bays")]
    pub drive_bays: Vec<HwGrokDriveBays>,
}

#[derive(Debug, Default, Deserialize)]
pub struct HwGrokPciDevices {
    pub label: String,
    #[serde(rename = "hc-fmri")]
    pub fmri: String,
    #[serde(rename = "pci-vendor-name")]
    pub pci_vendor_name: String,
    #[serde(rename = "pci-device-name")]
    pub pci_device_name: String,
    #[serde(rename = "pci-subsystem-name")]
    pub pci_subsystem_name: String,
    #[serde(rename = "device-path")]
    pub pci_device_path: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct HwGrokDriveBays {
    pub label: String,
    #[serde(rename = "hc-fmri")]
    pub fmri: String,
    pub disk: Option<HwGrokDisk>,
}

#[derive(Debug, Default, Deserialize)]
pub struct HwGrokDisk {
    #[serde(rename = "hc-fmri")]
    pub fmri: String,
    pub manufacturer: String,
    pub model: String,
    #[serde(rename = "serial-number")]
    pub serial_number: String,
    #[serde(rename = "firmware-revision")]
    pub disk_firmware_rev: String,
    #[serde(rename = "device-path")]
    pub disk_device_path: String,
}
