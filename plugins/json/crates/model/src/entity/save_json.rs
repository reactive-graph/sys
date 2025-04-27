use crate::ComponentSaveJson;
use crate::NAMESPACE_JSON;
use reactive_graph_graph::entity_ty;
use reactive_graph_std_base_model::Named;
use reactive_graph_sys_file_model::File;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_SAVE_JSON, NAMESPACE_JSON, ENTITY_TYPE_NAME_SAVE_JSON, "save_json");

entity_model!(
    SaveJson,
    set payload value,
);
impl ComponentSaveJson for SaveJson {}
impl File for SaveJson {}
impl Named for SaveJson {}
impl Action for SaveJson {}
