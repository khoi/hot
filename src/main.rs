use clap::Parser;
use metrics::Metrics;
use std::time::{SystemTime, UNIX_EPOCH};
mod metrics;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Use JSON output
    #[arg(long, default_value_t = false)]
    json: bool,
}

fn main() {
    let args = Args::parse();

    let metrics = get_metrics();
    if args.json {
        println!("{}", serde_json::to_string(&metrics).unwrap());
    } else {
        println!("System Metrics:");
        println!(
            "  Load averages: {:.2}, {:.2}, {:.2}",
            metrics.load_avg[0], metrics.load_avg[1], metrics.load_avg[2]
        );
        println!("  CPU temp avg: {:.1}°C", metrics.cpu_temp_avg);
        println!("  GPU temp avg: {:.1}°C", metrics.gpu_temp_avg);
        println!("  Battery: {}%", metrics.battery_percent);
    }
}

fn get_metrics() -> Metrics {
    Metrics {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64,
        load_avg: [1.0, 1.5, 2.0],
        cpu_temps: vec![45.0, 46.0, 44.0, 45.5],
        cpu_temp_avg: 45.125,
        cpu_percentages: vec![25.0, 30.0, 15.0, 20.0],
        cpu_power_wattage: 15.5,
        gpu_temps: vec![55.0],
        gpu_temp_avg: 55.0,
        gpu_percentages: vec![40.0],
        gpu_power_wattage: 25.0,
        ane_power_wattage: 0.5,
        battery_percent: 85.0,
    }
}
