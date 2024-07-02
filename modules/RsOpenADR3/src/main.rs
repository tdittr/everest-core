// EVerest expects binaries to be CamelCased, and Rust wants them to be snake_case. We yield to
// EVerest and shut up the compiler warning.
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use generated::get_config;
use std::{thread, time};
use std::sync::Arc;
use crate::generated::{Context, ExternalEnergyLimitsClientSubscriber, ModulePublisher, OnReadySubscriber};
use crate::generated::types::energy::{EnforcedLimits, ExternalLimits};

struct EverestInterface;

impl OnReadySubscriber for EverestInterface {
    fn on_ready(&self, pub_impl: &ModulePublisher) {
        pub_impl.r_limits.set_external_limits(ExternalLimits{ schedule_export: None, schedule_import: None }).unwrap();
    }
}

impl ExternalEnergyLimitsClientSubscriber for EverestInterface {
    fn on_enforced_limits(&self, _context: &Context, value: EnforcedLimits) {
        log::debug!("Consumer reported they are now enforcing: {:?}", value)
    }
}

fn main() {
    let config = get_config();
    println!("Received the config {config:?}");

    let interface = Arc::new(EverestInterface);
    let _module = generated::Module::new(interface.clone(), interface.clone());

    // Everest is driving execution in the background for us, nothing to do.
    loop {
        let dt = time::Duration::from_millis(250);
        thread::sleep(dt);
    }
}
