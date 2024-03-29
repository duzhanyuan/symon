# Symon [saɪmən]

Similar to neofetch, much faster, without the fancy ascii graphics, and written in rust.

This is a fork of [wojciechkepka/rustop](https://github.com/wojciechkepka/rustop)

## Prerequisites

- rustc 1.36.0 (a53f9df32 2019-07-03)
- Nightly optional, see comments in src/lib.rs
- Linux (tested on Arch Linux for now only)

## Installation

```bash
$ git clone https://github.com/aspera-non-spernit/symon
$ cd symon
$ cargo build --release
$ strip target/release/symon
# cp target/release/symon /usr/local/bin
# chmod 711 /usr/local/bin/symon
```
## Example output

**Note:** Example output may not display realistic values

```bash
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
```

# Benchmark

So fast.

```bash
$ hyperfine --warmup 3 "neofetch" "symon"
Benchmark #1: neofetch
  Time (mean ± σ):     504.5 ms ±   6.0 ms    [User: 315.6 ms, System: 172.0 ms]
  Range (min … max):   495.5 ms … 517.0 ms    10 runs
 
Benchmark #2: symon
  Time (mean ± σ):      25.8 ms ±   0.9 ms    [User: 21.4 ms, System: 4.3 ms]
  Range (min … max):    23.8 ms …  28.5 ms    93 runs
 
Summary
  'symon' ran
   19.56 ± 0.71 times faster than 'neofetch'
```
# Nice to know


### Grep

For now, use grep and sed to display a single system property.

```bash
$ symon | grep uptime --ignore-case | sed 's/├ UPTIME:           //'
```

### Super Graphics

Neofetch is cool. Try:

```bash
$ symon arch
```

:)