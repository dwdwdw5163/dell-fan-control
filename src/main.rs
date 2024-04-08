use tracing_subscriber;
use tracing::info;
use std::{time::Duration, borrow::BorrowMut};
use sensors::Sensors;



fn main() {
    tracing_subscriber::fmt::init();
    
    for chip in Sensors::new() {
        println!("Chip Name: {}", chip.get_name().unwrap());
        for (idx,feature) in chip.into_iter().enumerate() {
            if feature.feature_type() == &sensors::FeatureType::SENSORS_FEATURE_TEMP {
                println!("----Feature label {:?}", feature.get_label() );
            }
        }
        
    }
    
    
    
    
    loop {
        std::thread::sleep(Duration::from_secs(1));

        

    }
}
