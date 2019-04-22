# fm-log-report

This tool post-processes the aggregated, JSON-formatted FMA logs as produced by running:

```
% fmdump -AVj
```

and generates a summary report.   A file containing the output of [hwgrok](https://github.com/joyent/hwgrok) can optionally be specified.  If specified, it will be used to correlate the event telemetry with HW identity and location information to produce a more complete report.

Usage:

```
% fm_errlog_report -f <ERRLOG> [-H HWGROK]

Options:
    -h, --help            print this usage message
    -H, --hwgrok HWGROK   Output of hwgrok
    -f, --fmlog FMLOG     FM logs as JSON

```

Sample output:

```
===========================================================================
Device Path:                             /pci@0,0/pci8086,6f09@3,1/pci8086,4712@0
Vendor Name:                             Intel Corporation
Device Name:                             NVMe Datacenter SSD [3DNAND, Beta Rock Controller]
Subsystem Name:                          NVMe Datacenter SSD [3DNAND] ME 2.5  U.2 (P4600)
Total ereports:                          1

class                                    # occurences
-----                                    ------------
ereport.io.service.lost                  1

Event Occurrence Distribution
-----------------------------
2019-01-11                               1

===========================================================================
Device Path:                             /pci@0,0/pci8086,6f0a@3,2/pci8086,3703@0
Vendor Name:                             Intel Corporation
Device Name:                             PCIe Data Center SSD
Subsystem Name:                          DC P3700 SSD [2.5  SFF]
Total ereports:                          9

class                                    # occurences
-----                                    ------------
ereport.io.pci.nr                        4
ereport.io.pci.fabric                    4
ereport.io.service.lost                  1

Event Occurrence Distribution
-----------------------------
2019-01-04                               2
2019-01-11                               2
2019-01-15                               1
2019-02-25                               2
2019-03-05                               2

===========================================================================
Device Path:                             /pci@0,0/pci8086,6f06@2,2/pci15d9,808@0/iport@ff/disk@w5000cca26652bfb9,0
Disk Location:                           Slot11
Disk Manufacturer:                       HGST
Disk Model:                              HUH721010AL4204
Disk Serial:                             7JHGHT4G
Firmware Rev:                            C21D
Total ereports:                          2364

class                                    # occurences
-----                                    ------------
ereport.io.scsi.cmd.disk.recovered       1172
ereport.io.scsi.cmd.disk.dev.rqs.derr    4
ereport.io.scsi.cmd.disk.tran            1188

Event Occurrence Distribution
-----------------------------
2019-02-21                               16
2019-02-22                               22
2019-02-23                               14
2019-02-24                               13
2019-02-25                               22
2019-02-26                               12
2019-02-27                               6
2019-02-28                               12
2019-03-01                               10
2019-03-02                               16
2019-03-03                               28
2019-03-04                               48
2019-03-05                               44
2019-03-06                               24
2019-03-07                               56
2019-03-08                               56
2019-03-09                               61
2019-03-10                               72
2019-03-11                               22
2019-03-12                               24
2019-03-13                               36
2019-03-14                               116
2019-03-15                               40
2019-03-16                               49
2019-03-17                               22
2019-03-18                               50
2019-03-19                               53
2019-03-20                               42
2019-03-21                               30
2019-03-22                               53
2019-03-23                               68
2019-03-24                               18
2019-03-25                               52
2019-03-26                               67
2019-03-27                               90
2019-03-28                               52
2019-03-29                               30
2019-03-30                               34
2019-03-31                               8
2019-04-01                               18
2019-04-02                               42
2019-04-03                               19
2019-04-04                               42
2019-04-05                               58
2019-04-06                               68
2019-04-07                               38
2019-04-08                               44
2019-04-09                               28
2019-04-10                               32
2019-04-11                               36
2019-04-12                               28
2019-04-13                               48
2019-04-14                               73
2019-04-15                               93
2019-04-16                               47
2019-04-17                               70
2019-04-18                               92

```
