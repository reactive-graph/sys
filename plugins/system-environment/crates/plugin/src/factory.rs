use std::env;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::LazyLock;

use log::info;
use reactive_graph_graph::prelude::*;
use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_model_impl::ReactiveProperties;
use reactive_graph_reactive_service_api::ReactiveEntityRegistrationError;
use reactive_graph_runtime_model::COMPONENT_LABELED;
use reactive_graph_runtime_model::LabeledProperties::LABEL;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

use reactive_graph_std_base_model::COMPONENT_NAMED;
use reactive_graph_std_base_model::NamedProperties::NAME;
use reactive_graph_std_value_model::COMPONENT_VALUE;
use reactive_graph_std_value_model::ValueProperties::VALUE;

use reactive_graph_sys_system_environment_model::ENTITY_TYPE_SYSTEM_ENV_VAR;
use reactive_graph_sys_system_environment_model::NAMESPACE_SYSTEM_ENVIRONMENT_ID;

static SYSTEM_ENV_COMPONENTS: LazyLock<ComponentTypeIds> = LazyLock::new(|| {
    ComponentTypeIds::new()
        .component(COMPONENT_LABELED.clone())
        .component(COMPONENT_VALUE.clone())
        .component(COMPONENT_NAMED.clone())
});
#[derive(Component)]
pub struct SystemEnvironmentReactiveEntityFactory {
    #[component(default = "crate::plugin::inject_plugin_context")]
    context: Arc<dyn PluginContext + Send + Sync>,
}

#[derive(Debug, Error)]
pub enum SystemEnvironmentFactoryError {
    #[error("Entity type not found")]
    EntityTypeNotFound,
    #[error("Failed to create reactive entity: {0}")]
    ReactiveEntityRegistrationError(#[from] ReactiveEntityRegistrationError),
}

impl SystemEnvironmentReactiveEntityFactory {
    pub async fn create_entity_instances(&self) -> Result<(), SystemEnvironmentFactoryError> {
        let entity_type_manager = self.context.get_entity_type_manager();
        let ty = ENTITY_TYPE_SYSTEM_ENV_VAR.deref();
        // let ty = EntityTypeId::new_from_type("asdf", "asdf");
        info!("{ty}");
        let Some(entity_type) = entity_type_manager.get(ty) else {
            return Err(SystemEnvironmentFactoryError::EntityTypeNotFound);
        };
        let entity_instance_manager = self.context.get_entity_instance_manager();
        // let components = ComponentTypeIds::new()
        //     .component(&COMPONENT_LABELED)
        //     .component(&COMPONENT_VALUE)
        //     .component(&COMPONENT_NAMED);
        let properties = PropertyInstances::new_from_property_types_with_defaults(&entity_type.properties);
        for (name, value) in env::vars() {
            let id = Uuid::new_v5(&NAMESPACE_SYSTEM_ENVIRONMENT_ID, name.as_bytes());
            if entity_instance_manager.has(id) {
                continue;
            }
            let mut properties = properties.clone();
            properties.set(LABEL, json!(format!("/io/reactive-graph/system/env/{}", name.clone().to_lowercase())));
            properties.set(VALUE, json!(value));
            properties.set(NAME, json!(name.clone()));
            let properties = ReactiveProperties::new_with_id_from_properties(id, properties);
            let reactive_entity = ReactiveEntity::builder()
                // let entity_instance = EntityInstance::builder()
                .ty(&entity_type.ty)
                .id(id)
                .components(SYSTEM_ENV_COMPONENTS.clone())
                .properties(properties)
                .build();
            if let Err(e) = entity_instance_manager.register(reactive_entity) {
                return Err(e.into());
            }
        }
        Ok(())
    }
}
