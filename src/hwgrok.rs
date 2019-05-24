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
    pub chassis: HwGrokChassis,
    #[serde(rename = "pci-devices")]
    pub pci_devices: Vec<HwGrokPciDevices>,
    #[serde(rename = "drive-bays")]
    pub drive_bays: Vec<HwGrokDriveBay>,
    pub processors: Vec<HwGrokProcessors>,
    pub memory: Vec<HwGrokDimmSlot>,
    #[serde(rename = "service-processor")]
    pub sp: Option<HwGrokSP>,
    #[serde(rename = "power-supplies")]
    pub psus: Vec<HwGrokPSU>,
    pub fans: Vec<HwGrokFan>,
}

#[derive(Debug, Default, Deserialize)]
pub struct HwGrokSP {
    #[serde(rename = "hc-fmri")]
    pub sp_fmri: String,
    #[serde(rename = "firmware-revision")]
    pub sp_fw_version: String,
    #[serde(rename = "mac-address")]
    pub sp_mac_addr: String,
    #[serde(rename = "ipv4-address")]
    pub sp_ipv4_adrr: String,
    #[serde(rename = "ipv4-subnet")]
    pub sp_ipv4_subnet: String,
    #[serde(rename = "ipv4-gateway")]
    pub sp_ipv4_gateway: String,
    #[serde(rename = "ipv4-config-type")]
    pub sp_ipv4_cfg_type: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct HwGrokChassis {
    #[serde(rename = "hc-fmri")]
    pub chassis_fmri: String,
    #[serde(rename = "manufacturer")]
    pub chassis_manufacturer: String,
    #[serde(rename = "model")]
    pub chassis_model: String,
    #[serde(rename = "leds")]
    pub chassis_leds: Vec<HwGrokLED>,
}

#[derive(Debug, Default, Deserialize)]
pub struct HwGrokProcessors {
    #[serde(rename = "hc-fmri")]
    pub chip_fmri: String,
    #[serde(rename = "label")]
    pub chip_label: String,
    #[serde(rename = "processor-brand")]
    pub chip_brand: String,
    #[serde(rename = "leds")]
    pub chip_leds: Vec<HwGrokLED>,
}

#[derive(Debug, Default, Deserialize)]
pub struct HwGrokDimmSlot {
    #[serde(rename = "hc-fmri")]
    pub slot_fmri: String,
    #[serde(rename = "label")]
    pub slot_label: String,
    pub dimm: Option<HwGrokDimm>,
}

#[derive(Debug, Default, Deserialize)]
pub struct HwGrokDimm {
    #[serde(rename = "hc-fmri")]
    pub dimm_fmri: String,
    #[serde(rename = "manufacturer")]
    pub dimm_manufacturer: String,
    #[serde(rename = "part-number")]
    pub dimm_part: String,
    #[serde(rename = "type")]
    pub dimm_type: String,
    #[serde(rename = "size-in-bytes")]
    pub dimm_size: u64,
    #[serde(rename = "leds")]
    pub dimm_leds: Vec<HwGrokLED>,
}

#[derive(Debug, Default, Deserialize)]
pub struct HwGrokPciDevices {
    #[serde(rename = "hc-fmri")]
    pub pci_fmri: String,
    #[serde(rename = "label")]
    pub pci_label: String,
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
pub struct HwGrokDriveBay {
    #[serde(rename = "hc-fmri")]
    pub bay_fmri: String,
    #[serde(rename = "label")]
    pub bay_label: String,
    #[serde(rename = "disk")]
    pub bay_disk: Option<HwGrokDisk>,
    #[serde(rename = "leds")]
    pub bay_leds: Vec<HwGrokLED>,
}

#[derive(Debug, Default, Deserialize)]
pub struct HwGrokDisk {
    #[serde(rename = "hc-fmri")]
    pub disk_fmri: String,
    #[serde(rename = "manufacturer")]
    pub disk_manufacturer: String,
    #[serde(rename = "model")]
    pub disk_model: String,
    #[serde(rename = "serial-number")]
    pub disk_serial_number: String,
    #[serde(rename = "firmware-revision")]
    pub disk_firmware_rev: String,
    #[serde(rename = "device-path")]
    pub disk_device_path: String,
    #[serde(rename = "size-in-bytes")]
    pub disk_size: u64,
    #[serde(rename = "speed-in-rpm")]
    pub disk_rpm: Option<u64>,
}

#[derive(Debug, Default, Deserialize)]
pub struct HwGrokPSU {
    #[serde(rename = "hc-fmri")]
    pub psu_fmri: String,
    #[serde(rename = "label")]
    pub psu_label: String,
    #[serde(rename = "manufacturer")]
    pub psu_manufacturer: String,
    #[serde(rename = "model")]
    pub psu_model: String,
    #[serde(rename = "firmware-revision")]
    pub psu_firmware_rev: String,
    #[serde(rename = "leds")]
    pub psu_leds: Vec<HwGrokLED>,
}

#[derive(Debug, Default, Deserialize)]
pub struct HwGrokFan {
    #[serde(rename = "hc-fmri")]
    pub fan_fmri: String,
    #[serde(rename = "label")]
    pub fan_label: String,
    #[serde(rename = "leds")]
    pub fan_leds: Vec<HwGrokLED>,
}

#[derive(Debug, Default, Deserialize)]
pub struct HwGrokLED {
    #[serde(rename = "type")]
    pub led_type: String,
    #[serde(rename = "mode")]
    pub led_mode: String,
}
