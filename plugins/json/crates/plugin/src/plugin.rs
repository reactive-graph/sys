use reactive_graph_plugin_api::EntityComponentBehaviourRegistry;
use reactive_graph_plugin_api::prelude::plugin::*;
use reactive_graph_plugin_api::prelude::providers::*;

use crate::behaviour::component::load_json::LoadJsonFactory;
use crate::behaviour::component::save_json::SaveJsonFactory;
use reactive_graph_sys_json_model::BEHAVIOUR_LOAD_JSON;
use reactive_graph_sys_json_model::BEHAVIOUR_SAVE_JSON;
use reactive_graph_sys_json_model::COMPONENT_BEHAVIOUR_LOAD_JSON;
use reactive_graph_sys_json_model::COMPONENT_BEHAVIOUR_SAVE_JSON;

export_plugin!({
    "dependencies": [
        { "name": "reactive-graph-std-base", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-std-result", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-std-trigger", "version": ">=0.10.0, <0.11.0" },
        { "name": "reactive-graph-sys-file", "version": ">=0.10.0, <0.11.0" }
    ]
});

#[injectable]
pub trait JsonPlugin: Plugin + Send + Sync {}

#[derive(Component)]
pub struct JsonPluginImpl {
    component_provider: Arc<dyn TypeProvider<Components> + Send + Sync>,

    #[component(default = "component_provider_registry")]
    component_provider_registry: Arc<dyn ComponentProviderRegistry + Send + Sync>,

    entity_types_provider: Arc<dyn TypeProvider<EntityTypes> + Send + Sync>,

    #[component(default = "entity_types_provider_registry")]
    entity_type_provider_registry: Arc<dyn EntityTypeProviderRegistry + Send + Sync>,

    #[component(default = "entity_component_behaviour_registry")]
    entity_component_behaviour_registry: Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl Plugin for JsonPluginImpl {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.component_provider_registry.register_provider(self.component_provider.clone()).await;
        self.entity_type_provider_registry.register_provider(self.entity_types_provider.clone()).await;

        // Load Json
        let factory = Arc::new(LoadJsonFactory::new(BEHAVIOUR_LOAD_JSON.clone()));
        self.entity_component_behaviour_registry
            .register(COMPONENT_BEHAVIOUR_LOAD_JSON.clone(), factory)
            .await;

        // Save Json
        let factory = Arc::new(SaveJsonFactory::new(BEHAVIOUR_SAVE_JSON.clone()));
        self.entity_component_behaviour_registry
            .register(COMPONENT_BEHAVIOUR_SAVE_JSON.clone(), factory)
            .await;

        Ok(())
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_LOAD_JSON).await;
        self.entity_component_behaviour_registry.unregister(&COMPONENT_BEHAVIOUR_SAVE_JSON).await;

        self.entity_type_provider_registry.unregister_provider(self.entity_types_provider.id()).await;
        self.component_provider_registry.unregister_provider(self.component_provider.id()).await;
        Ok(())
    }
}
