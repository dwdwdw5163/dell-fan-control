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
    info!("Set PWM to {}", target);
}

#[tracing::instrument]
fn init() {
    std::process::Command::new("ipmitool")
        .args(["sdr", "type", "fan"])
        .spawn()
        .expect("failed to execute process");
    std::process::Command::new("ipmitool")
        .args(["sdr", "type", "temp"])
        .spawn()
        .expect("failed to execute process");
}

fn main() {
    tracing_subscriber::fmt::init();
    init();

    let mut context = Context::default();

    loop {
        std::thread::sleep(Duration::from_secs(1));
        let temp = compute_temp();
        let temp_level = (temp as u32 / 10) * 10;
        if temp_level > context.current_temp_level {
            context.current_temp_level = temp_level;
            set_pwm((context.current_temp_level - 10).max(0));
        } else if temp < (context.current_temp_level as f64 - HYSTERESIS) {
            context.current_temp_level = temp_level;
            set_pwm((context.current_temp_level - 10).max(0));
        } else {
        }
    }
}
