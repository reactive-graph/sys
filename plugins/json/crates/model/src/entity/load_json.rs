use crate::ComponentLoadJson;
use crate::NAMESPACE_JSON;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;
use reactive_graph_std_base_model::Named;
use reactive_graph_std_result_model::ResultAny;
use reactive_graph_sys_file_model::File;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_LOAD_JSON, NAMESPACE_JSON, ENTITY_TYPE_NAME_LOAD_JSON, "load_json");

entity_model!(LoadJson);
impl ComponentLoadJson for LoadJson {}
impl File for LoadJson {}
impl Action for LoadJson {}
impl Named for LoadJson {}
impl ResultAny for LoadJson {}
