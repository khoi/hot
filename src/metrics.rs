use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metrics {
    pub timestamp: i64,

    pub load_avg: [f64; 3],

    pub cpu_temps: Vec<f64>,
    pub cpu_temp_avg: f64,
    pub cpu_percentages: Vec<f64>,
    pub cpu_power_wattage: f64,

    pub gpu_temps: Vec<f64>,
    pub gpu_temp_avg: f64,
    pub gpu_percentages: Vec<f64>,
    pub gpu_power_wattage: f64,

    pub ane_power_wattage: f64,

    pub battery_percent: f64,
}
