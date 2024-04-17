use sensors::Sensors;
use std::time::Duration;
use tracing::info;
use tracing_subscriber;

#[derive(Default)]
struct Context {
    current_temp_level: u32,
}

const HYSTERESIS: f64 = 3.0;

#[tracing::instrument]
fn compute_temp() -> f64 {
    let mut temps = vec![];
    for chip in Sensors::new() {
        for (idx, feature) in chip.into_iter().enumerate() {
            if feature.feature_type() == &sensors::FeatureType::SENSORS_FEATURE_TEMP {
                for subfeature in feature {
                    if subfeature.subfeature_type()
                        == &sensors::SubfeatureType::SENSORS_SUBFEATURE_TEMP_INPUT
                    {
                        temps.push(subfeature.get_value().unwrap());
                    }
                }
            }
        }
    }
    let avg = temps.iter().sum::<f64>() / temps.len() as f64;
    info!("Avg Temp: {:.2}", avg);
    avg
}

#[tracing::instrument]
fn set_pwm(target: u32) {
    let target_hex = format!("{:#04x}", target);
    info!("Set PWM to {}({})", target, target_hex);
    // std::process::Command::new("ipmitool")
    //     .args(["raw", "0x30", "0x30", "0x02", "0xff", &target_hex])
    //     .status()
    //     .expect("failed to execute process");
    
}

#[tracing::instrument]
fn impi_info() {
    info!("Enable Manual Fan Control");
    //enable manual fan control
    std::process::Command::new("ipmitool")
        .args(["raw", "0x30", "0x30", "0x01", "0x00"])
        .status()
        .expect("failed to execute process");
    
    //Read fan speed
    std::process::Command::new("ipmitool")
        .args(["sdr", "type", "fan"])
        .status()
        .expect("failed to execute process");

    //Read Temp
    std::process::Command::new("ipmitool")
        .args(["sdr", "type", "temp"])
        .status()
        .expect("failed to execute process");
}

fn main() {
    tracing_subscriber::fmt::init();
    impi_info();

    let mut context = Context::default();

    loop {
        std::thread::sleep(Duration::from_secs(1));
        let temp = compute_temp();
        let temp_level = (temp as u32 / 10) * 10;
        if temp_level > context.current_temp_level {
            context.current_temp_level = temp_level;
            set_pwm((context.current_temp_level - 10).max(0));
            impi_info();
        } else if temp < (context.current_temp_level as f64 - HYSTERESIS) {
            context.current_temp_level = temp_level;
            set_pwm((context.current_temp_level - 10).max(0));
            impi_info();
        } else {
        }
    }
}
