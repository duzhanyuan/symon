use regex::Regex;
use std::{fmt, fs, thread, time};
use std::collections::HashMap;

#[derive(Debug)]
struct NetworkDevice {
    name: String,
    received_bytes: u64,
    transfered_bytes: u64,
}

impl NetworkDevice {
    fn new() -> NetworkDevice {
        NetworkDevice {
            name: String::from(""),
            received_bytes: 0,
            transfered_bytes: 0
        }
    }
}

impl fmt::Display for NetworkDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"
│   ├─ {}
│   │     DOWN:     {}      {}
│   │     UP:       {}      {}",
            self.name,
            conv_b(self.received_bytes), self.received_bytes,
            conv_b(self.transfered_bytes), self.transfered_bytes
        )
    }
}

#[derive(Debug)]
struct Partition {
    name: String,
    major: u8,
    minor: u8,
    size: u64,
    filesystem: String,
    mountpoint: String
}

impl Partition {
    fn new() -> Partition {
        Partition {
            name: String::from(""),
            major: 0,
            minor: 0,
            size: 0,
            filesystem: String::from(""),
            mountpoint: String::from("")
        }
    }
}

impl fmt::Display for Partition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"
    ├── {}
    │     MAJ:MIN:     {}:{}
    │     SIZE:        {}    {}
    │     FILESYSTEM:  {}
    │     MOUNTPOINT:  {}", 
            self.name,
            self.major, self.minor,
            conv_b(self.size), self.size,
            self.filesystem,
            self.mountpoint
        )
    }
}

#[derive(Debug)]
struct Storage {
    name: String,
    major: u8,
    minor: u8,
    size: u64,
}

impl Storage {
    fn new() -> Storage {
        Storage {
            name: String::from(""),
            major: 0,
            minor: 0,
            size: 0,
        }
    }
}

impl fmt::Display for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"        
│   ├─ {}
│   │     MAJ:MIN:     {}:{}
│   │     SIZE:        {}    {}",
            self.name,
            self.major, self.minor,
            conv_b(self.size), self.size
        )
    }
}


#[derive(Debug)]
pub struct PcInfo {
    hostname: String,
    kernel_version: String,
    uptime: f64,
    cpu: String,
    cpu_clock: f32,
    memory: u64,
    free_memory: u64,
    swap: u64,
    free_swap: u64,
    network_dev: Vec<NetworkDevice>,
    storage_dev: Vec<Storage>,
    partitions: HashMap<String, Vec<Partition>>
}

impl PcInfo {
    pub fn new() -> PcInfo {
        PcInfo {
            hostname: Process::get(SystemProperty::Hostname),
            kernel_version: Process::get(SystemProperty::OsRelease),
            uptime: Process::uptime(),
            cpu: Process::cpu_info(),
            cpu_clock: Process::cpu_clock(),
            memory: Process::memory_total(),
            free_memory: Process::memory_free(),
            swap: Process::swap_total(),
            free_swap: Process::swap_free(),
            network_dev: Process::network_dev(),
            storage_dev: Process::storage_dev(),
            partitions: Process::storage_partitions(Process::storage_dev())
        }
    }
}

impl fmt::Display for PcInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut networks = String::new();
        for interface in &self.network_dev {
            networks.push_str(&interface.to_string());
        }
        let mut storage = String::new();
        for store in &self.storage_dev {
            storage.push_str(&store.to_string());
        }
        let mut partitions = String::new();
        for parts in self.partitions.values() {
            for p in parts {
                partitions.push_str(&p.to_string());
            }
      //      dbg!(part);
        }
        write!(f, 
        "┌── SYSTEM INFORMATION ──────
├ HOSTNAME:         {}
├ KERNEL VERSION:   {}
├ UPTIME:           {}
├ CPU:              {}
├ CPU CLOCK:        {:.2} MHz
├ MEM:              {}  {}
├ MEMFREE:          {}  {}  {}%
├ SWAP:             {}   {}
├ SWAPFREE:         {}   {}  {}%
├ NETWORK DEVICE: {}
├ STORAGE: {}
└ PARTITIONS: {}"
        ,   self.hostname, self.kernel_version, self.uptime, self.cpu,
            self.cpu_clock,
            conv_b(self.memory), self.memory,
            conv_b(self.free_memory), self.free_memory, conv_p(self.memory, self.free_memory),
            conv_b(self.swap), self.swap, 
            conv_b(self.free_swap), self.free_swap, conv_p(self.swap, self.free_swap),
            networks, storage, partitions )
    }
}

#[derive(Debug)]
enum SystemProperty { Hostname, OsRelease }
struct Process;
impl Process {
    fn get(prop: SystemProperty) -> String {
        let mut path = String::from("/proc/sys/kernel/");
        path.push_str( match prop {
                SystemProperty::Hostname => "hostname",
                SystemProperty::OsRelease => "osrelease",
                _ => "null"
            }
        );
        path
    }

    fn uptime() -> f64 {
        match fs::read_to_string("/proc/uptime") {
            Ok(res) => {
                let data: Vec<&str> = res.split(' ').collect();
                match data[0].parse::<f64>() {
                    Ok(n) => n,
                    _ => 0.
                }
            },
            _ => 0.
        }
    }

    fn memory_total() -> u64{
        match fs::read_to_string("/proc/meminfo") {
            Ok(res) => {
                let re = Regex::new(r"MemTotal:\s*(\d*)").unwrap();
                let data = re.captures(&res).unwrap();
                match data[1].parse::<u64>() {
                    Ok(n) => n*1024,
                    Err(e) => {
                        println!("{}", e);
                        0
                    }
                }
            },
            _ => 0
        }
    }

    fn memory_free() -> u64 {
        match fs::read_to_string("/proc/meminfo") {
            Ok(res) => {
                let re = Regex::new(r"MemFree:\s*(\d*)").unwrap();
                let data = re.captures(&res).unwrap();
                match data[1].parse::<u64>() {
                    Ok(n) => n*1024,
                    Err(e) => {
                        println!("{}", e);
                        0
                    }
                }
            },
            _ => 0
        }
    }

    fn swap_total() -> u64 {
        match fs::read_to_string("/proc/meminfo") {
            Ok(res) => {
                let re = Regex::new(r"SwapTotal:\s*(\d*)").unwrap();
                let data = re.captures(&res).unwrap();
                match data[1].parse::<u64>() {
                    Ok(n) => n*1024,
                    Err(e) => {
                        println!("{}", e);
                        0
                    }
                }
            },
            _ => 0
        }
    }

    fn swap_free() -> u64 {
        match fs::read_to_string("/proc/meminfo") {
            Ok(res) => {
                let re = Regex::new(r"SwapFree:\s*(\d*)").unwrap();
                let data = re.captures(&res).unwrap();
                match data[1].parse::<u64>() {
                    Ok(n) => n*1024,
                    Err(e) => {
                        println!("{}", e);
                        0
                    }
                }
            },
            _ => 0
        }
    }

    fn cpu_info() -> String {
        match fs::read_to_string("/proc/cpuinfo") {
            Ok(res) => {
                let re = Regex::new(r"model name\s*: (.*)").unwrap();
                let data = re.captures(&res).unwrap();
                String::from(&data[1])
            },
            Err(e) => {
                println!("Error - {}", e);
                String::from("")
            }
        }
    }

    fn cpu_clock() -> f32 {
        match fs::read_to_string("/proc/cpuinfo") {
            Ok(res) => {
                // println!("{}", res);
                let re = Regex::new(r"cpu MHz\s*: (.*)").unwrap();
                let mut clock_speed = 0.;
                let mut core_count = 0;
                for core_clock in re.captures_iter(&res) {
                    match &core_clock[1].parse::<f32>() {
                        Ok(n) => {
                            clock_speed += n;
                            core_count += 1;
                        }
                        Err(e) => println!("Error - {}", e)
                    }
                }
                clock_speed / core_count as f32
            },
            Err(e) => {
                println!("Error - {}", e);
                0.
            }
        }
    }

    fn network_dev() -> Vec<NetworkDevice> {
        let mut devices = Vec::new();
        match fs::read_to_string("/proc/net/dev") {
            Ok(res) => {
                let re = Regex::new(r"([\d\w]*):\s*(\d*)\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*\d*\s*(\d*)").unwrap();
                for network_dev in re.captures_iter(&res) {
                    let mut interface = NetworkDevice::new();
                    let received = match network_dev[2].parse::<u64>() {
                        Ok(n) => n,
                        _ => 0
                    };
                    let transfered = match network_dev[3].parse::<u64>() {
                        Ok(n) => n,
                        _ => 0
                    };
                    interface.name = String::from(&network_dev[1]);
                    interface.received_bytes = received;
                    interface.transfered_bytes = transfered;
                    devices.push(interface);
                }
                devices
            },
            Err(e) => {
                println!("Error - {}", e);
                devices
            }
        }
    }

    fn storage_dev() -> Vec<Storage> {
        let mut devices = Vec::new();
        match fs::read_to_string("/proc/partitions") {
            Ok(res) => {
                let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s(\D*)$").unwrap();
                for storage_dev in re.captures_iter(&res) {
                    let mut storage = Storage::new();
                    let major = match storage_dev[1].parse::<u8>() {
                        Ok(n) => n,
                        _ => 0
                    };
                    let minor = match storage_dev[2].parse::<u8>() {
                        Ok(n) => n,
                        _ => 0
                    };
                    let blocks = match storage_dev[3].parse::<u64>() {
                        Ok(n) => n,
                        _ => 0
                    };
                    let storage_name = &storage_dev[4];
                    storage.name = String::from(storage_name);
                    storage.major = major;
                    storage.minor = minor;
                    storage.size = blocks*1024;
                    devices.push(storage);
                }
                devices
            },
            Err(e) => {
                println!("Error - {}", e);
                devices
            }
        }
    }

    fn storage_partitions(storage_dev: Vec<Storage>) -> HashMap<String, Vec<Partition>> {
        let mut devices = HashMap::new();
        match fs::read_to_string("/proc/partitions") {
            Ok(res) => {
                for dev in &storage_dev{
                    let dev_name = &dev.name;
                    let mut partitions = Vec::new();
                    let re = Regex::new(r"(?m)^\s*(\d*)\s*(\d*)\s*(\d*)\s(\w*\d+)$").unwrap();
                    for storage_dev in re.captures_iter(&res) {
                        if &storage_dev[4][..3] == dev_name {
                            let mut partition = Partition::new();
                            let major = match storage_dev[1].parse::<u8>() {
                                Ok(n) => n,
                                _ => 0
                            };
                            let minor = match storage_dev[2].parse::<u8>() {
                                Ok(n) => n,
                                _ => 0
                            };
                            let blocks = match storage_dev[3].parse::<u64>() {
                                Ok(n) => n,
                                _ => 0
                            };
                            let partition_name = &storage_dev[4];

                            match fs::read_to_string("/proc/mounts") {
                                Ok(data) => {
                                    let rere = Regex::new(r"/dev/(\w*)\s(\S*)\s(\S*)").unwrap();
                                    for found_partition in rere.captures_iter(&data) {
                                        if &found_partition[1] == partition_name {
                                            let mountpoint = &found_partition[2];
                                            let filesystem = &found_partition[3];
                                            partition.mountpoint = String::from(mountpoint);
                                            partition.filesystem = String::from(filesystem);
                                            break;
                                        }
                                        else {
                                            partition.mountpoint = String::from("");
                                            partition.filesystem = String::from("");
                                        }
                                        
                                    }
                                    partition.name = String::from(partition_name);
                                    partition.major = major;
                                    partition.minor = minor;
                                    partition.size = blocks*1024;
                                    partitions.push(partition);
                                }
                                Err(e) => {
                                    println!("{}", e);
                                    partition.mountpoint = String::from("");
                                    partition.filesystem = String::from("");
                                }
                            }           
                        }
                    }
                    devices.insert(String::from(dev_name), partitions);
                }
                devices
            },
            Err(e) => {
                println!("Error - {}", e);
                devices
            }
        }
    }
}

fn conv_p(total: u64, free: u64) -> u64 {
    if total != 0 {
        free * 100 / total
    } else {
        0
    }
}

fn conv_b(bytes: u64) -> String {
    let n: f64 = bytes as f64;
    if n < 1024. {
        format!("{} B", n)
    }
    else if 1024. <= n && n < u64::pow(1024, 2) as f64 {
        let s = n / 1024.;
        format!("{:.2} KB", s)
    }
    else if u64::pow(1024, 2) as f64 <= n && n < u64::pow(1024, 3) as f64 {
        let s = n / u64::pow(1024, 2) as f64;
        format!("{:.2} MB", s)
    }
    else if u64::pow(1024, 3) as f64 <= n && n < u64::pow(1024, 4) as f64 {
        let s = n / u64::pow(1024, 3) as f64;
        format!("{:.2} GB", s)
    }
    else {
        let s = n / u64::pow(1024, 4) as f64;
        format!("{:.2} TB", s)
    }
}

// unused
// fn conv_t(sec: f64) -> String {
//     if sec < 60. {
//         format!("{} seconds", sec)
//     }
//     else if 60. <= sec && sec < u64::pow(60, 2) as f64{
//         let minutes = (sec / 60.).floor();
//         let seconds = (sec % 60.).floor();
//         format!("{} minutes {} seconds", minutes, seconds)
//     }
//     else if u64::pow(60, 2) as f64 <= sec && sec < u64::pow(60, 3) as f64{
//         let hours = (sec / u64::pow(60, 2) as f64).floor();
//         let minutes = ((sec % u64::pow(60, 2) as f64) / 60.).floor();
//         let seconds = ((sec % u64::pow(60, 2) as f64) % 60.).floor();
//         format!("{} hours {} minutes {} seconds", hours, minutes, seconds)
//     }
//     else {
//         let days = (sec / (u64::pow(60, 2) as f64 * 24.)).floor();
//         let hours = ((sec % (u64::pow(60, 2) as f64 * 24.)) / u64::pow(60, 2) as f64).floor();
//         let minutes = (((sec % (u64::pow(60, 2) as f64 * 24.)) % u64::pow(60, 2) as f64) / 60.).floor();
//         let seconds = (((sec % (u64::pow(60, 2) as f64 * 24.)) % u64::pow(60, 2) as f64) % 60.).floor();
//         format!("{} days {} hours {} minutes {} seconds", days, hours, minutes, seconds)
//     }
// }