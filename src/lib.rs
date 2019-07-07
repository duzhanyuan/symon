mod utils;
use regex::Regex;
use std::{ fmt, fs };
use std::collections::HashMap;

#[derive(Debug)] enum SystemProperty { CpuInfo, Hostname, MemInfo, OsRelease, Uptime }

#[derive(Debug, Default)] struct MemInfo { memory: u64, free: u64, swap: u64, swap_free: u64 }
#[derive(Debug)] struct NetworkDevice { name: String, received_bytes: u64, transfered_bytes: u64 }
#[derive(Debug)] 
struct Partition { name: String, major: u8,  minor: u8, size: u64, filesystem: String, mountpoint: String }
#[derive(Debug, Default)]
pub struct PcInfo {
    hostname: String,
    kernel_version: String,
    uptime: String,
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
struct Process;
#[derive(Debug)] struct Storage { name: String, major: u8, minor: u8,  size: u64 }
#[derive(Debug, Default)] struct Uptime { first: f64, second: f64 }

impl From<String> for MemInfo { 
    fn from(string: String) -> MemInfo {
        let rgx = vec![
            Regex::new(r"MemTotal:\s*(\d*)"),
            Regex::new(r"MemFree:\s*(\d*)"),
            Regex::new(r"SwapTotal:\s*(\d*)"),
            Regex::new(r"SwapFree:\s*(\d*)")
        ];
        let mut fields = vec![];
        for r in rgx.iter().take(4) {
            let data = r.clone().unwrap().captures(&string).unwrap();
            match data[1].parse::<u64>() {
                Ok(n) =>  fields.push(n * 1024),
                Err(_e) => fields.push(0)
            }
        }
        MemInfo { memory: fields[0], free: fields[1], swap: fields[2], swap_free: fields[3] }
    }
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
            utils::conv_b(self.received_bytes), self.received_bytes,
            utils::conv_b(self.transfered_bytes), self.transfered_bytes
        )
    }
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
            utils::conv_b(self.size), self.size,
            self.filesystem,
            self.mountpoint
        )
    }
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
            utils::conv_b(self.size), self.size
        )
    }
}

impl From<String> for Uptime {
    fn from(string: String) -> Self {
        let data: Vec<&str> = string.split(' ').collect();
        Uptime { first: data[0].parse().unwrap(), second: 0.0 }
    }
}
impl Into<String> for Uptime { fn into(self) -> String { utils::conv_t(self.first) } }

impl PcInfo {
    pub fn new() -> PcInfo {
        let mem_info = MemInfo::from(Process::get(SystemProperty::MemInfo));
        PcInfo {
            hostname: Process::get(SystemProperty::Hostname),
            kernel_version: Process::get(SystemProperty::OsRelease),
            uptime: Uptime::from(Process::get(SystemProperty::Uptime)).into(),
            cpu: Process::cpu_info(),
            cpu_clock: Process::cpu_clock(),
            memory: mem_info.memory,
            free_memory: mem_info.free,
            swap: mem_info.swap,
            free_swap: mem_info.swap_free,
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
        }
        write!(f, 
"┌── SYSTEM INFORMATION ──────
├ HOSTNAME:         {}
├ KERNEL VERSION:   {}
├ UPTIME:           {:#?}
├ CPU:              {}
├ CPU CLOCK:        {:.2} MHz
├ MEM:              {}  {}
├ MEMFREE:          {}  {}  {}%
├ SWAP:             {}   {}
├ SWAPFREE:         {}   {}  {}%
├ NETWORK DEVICE: {}
├ STORAGE: {}
└ PARTITIONS: {}"
        , self.hostname, self.kernel_version, self.uptime, self.cpu,
        self.cpu_clock,
        utils::conv_b(self.memory), self.memory,
        utils::conv_b(self.free_memory), self.free_memory, utils::conv_p(self.memory, self.free_memory),
        utils::conv_b(self.swap), self.swap, 
        utils::conv_b(self.free_swap), self.free_swap, utils::conv_p(self.swap, self.free_swap),
        networks, storage, partitions )
    }
}

impl Process {
    fn get(prop: SystemProperty) -> String {
        let mut path = String::from("/proc/");
        path.push_str( match prop {
                SystemProperty::CpuInfo => "cpuinfo",
                SystemProperty::MemInfo => "meminfo",
                SystemProperty::Hostname => "sys/kernel/hostname",
                SystemProperty::OsRelease => "sys/kernel/osrelease",
                SystemProperty::Uptime => "uptime"
            }
        );
        fs::read_to_string(path).unwrap()
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