pub const BANNER: &str = r"
  ____                    _
 / __ \                  | |
| |  | |_   _  __ _ _ __ | |_ __ _
| |  | | | | |/ _` | '_ \| __/ _` |
| |__| | |_| | (_| | | | | || (_| |
 \___\_\\__,_|\__,_|_| |_|\__\__,_|";

#[derive(Debug)]
pub struct Machine {
    /// Hostname of the machine
    pub hostname: String,

    /// Cpu usage in percentage
    pub cpu: u16,

    /// Ram usage in gigabytes
    /// The tuples is for minimum and maximum values
    pub ram: (f32, f32),

    /// Gpu usage in percentage
    pub gpu_usage: u32,
    pub gpu_power: u32,

    /// Temperature in Celcius
    pub temperature: f32,

    /// The owner of the machine
    pub owner: String,
}