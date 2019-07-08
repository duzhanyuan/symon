# Rustop
All important information about your system in one place.

This is a fork of [wojciechkepka/rustop](https://github.com/wojciechkepka/rustop)

## Prerequisites

- rustc 1.36.0 (a53f9df32 2019-07-03)
- Nightly optional, see comments in src/lib.rs
- Linux (tested on Arch Linux for now only)

## Installation

```bash
$ git clone https://github.com/aspera-non-spernit/rustop
$ cd rustop
$ cargo build --release
$ strip target/release/rustop
# cp target/release/rustop /usr/local/bin
# chmod 711 /usr/local/bin/rustop
```
## Example output

**Note:** Example output may not display realistic values

┌── SYSTEM INFORMATION ──────
├ HOSTNAME:         jflkefl2f
├ KERNEL VERSION:   5.1.16-arch1-1-ARCH
├ UPTIME:           "0 days, 0 hours 58 minutes 3494 seconds."
├ CPU:              AMD FX(tm)-8150 Eight-Core Processor
├ CPU CLOCK:        4010.42 MHz
├ MEM:              15.64 GB  16789274624
├ MEMFREE:          10.61 GB  11393101824  67%
├ SWAP:             8.00 GB   8589930496
├ SWAPFREE:         8.00 GB   8589930496  100%
├ NETWORK DEVICE: 
│   ├─ wlp3s0
│   │     DOWN:     16.44 MB      17243357
│   │     UP:       2.38 MB      2499074
│   ├─ lo
│   │     DOWN:     6.42 KB      6574
│   │     UP:       6.42 KB      6574
│   ├─ enp2s0
│   │     DOWN:     0.00 B      0
│   │     UP:       0.00 B      0
├ STORAGE:         
│   ├─ sda
│   │     MAJ:MIN:     8:0
│   │     SIZE:        1.82 TB    2000398934016      
│   ├─ sdb
│   │     MAJ:MIN:     8:16
│   │     SIZE:        14.84 GB    15931539456
└ PARTITIONS: 
    ├── sdb1
    │     MAJ:MIN:     8:17
    │     SIZE:        14.84 GB    15930473472
    │     FILESYSTEM:  ext4
    │     MOUNTPOINT:  /run/media/aspera-non-spernit/42411f25-7173-4d07-ba4e-a2b41535b931
    ├── sda1
    │     MAJ:MIN:     8:1
    │     SIZE:        256.00 MB    268435456
    │     FILESYSTEM:  vfat
    │     MOUNTPOINT:  /boot
    ├── sda2
    │     MAJ:MIN:     8:2
    │     SIZE:        111.54 GB    119764622336
    │     FILESYSTEM:  ext4
    │     MOUNTPOINT:  /
