use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use serde_json::Value;
use uuid::Uuid;

use reactive_graph_sys_binary_model::BinaryDataProperties::DATA_URL;
use reactive_graph_sys_file_model::FileProperties::FILENAME;

entity_behaviour!(
    SaveBinaryData,
    SaveBinaryDataFactory,
    SaveBinaryDataFsm,
    SaveBinaryDataBehaviourTransitions,
    SaveBinaryDataValidator
);

behaviour_validator!(SaveBinaryDataValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), FILENAME.as_ref(), DATA_URL.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for SaveBinaryDataBehaviourTransitions {}

impl BehaviourConnect<Uuid, ReactiveEntity> for SaveBinaryDataBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            if let Some(filename) = reactive_instance.as_string(FILENAME) {
                let filename = shellexpand::tilde(&filename);
                let path = Path::new(filename.as_ref());

                if let Some(data_url) = reactive_instance.get(DATA_URL).and_then(|v| v.as_str().map(String::from)) {
                    // The next operations may be performance intensive
                    // Decode DataURL with BASE64 encoding to byte array
                    let mut parts = data_url.splitn(2, ',');
                    let _part_data_url_prefix = parts.next();
                    let bytes = match parts.next() {
                        Some(part_base64_encoded_data) => STANDARD.decode(part_base64_encoded_data).ok(),
                        None => None,
                    };
                    if bytes.is_none() {
                        return;
                    }

                    // Write byte array to file
                    if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open(path) {
                        let _success = file.write_all(bytes.unwrap().as_slice());
                    }
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for SaveBinaryDataBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for SaveBinaryDataBehaviourTransitions {}
