use reactive_graph_plugin_api::prelude::providers::*;

#[derive(TypeProvider, Component)]
#[type_provider(tys = "Components", path = "types/components")]
pub struct ConfigComponentsProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "EntityTypes", path = "types/entities")]
pub struct ConfigEntityTypesProvider {}
