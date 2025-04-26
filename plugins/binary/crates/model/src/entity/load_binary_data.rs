use crate::BinaryData;
use crate::BinaryDataUrl;
use crate::NAMESPACE_BINARY;
use reactive_graph_graph::entity_ty;
use reactive_graph_model_base::Named;
use reactive_graph_sys_file_model::File;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_LOAD_BINARY_DATA, NAMESPACE_BINARY, ENTITY_TYPE_NAME_LOAD_BINARY_DATA, "load_binary_data");

entity_model!(LoadBinaryData);
impl BinaryData for LoadBinaryData {}
impl BinaryDataUrl for LoadBinaryData {}
impl File for LoadBinaryData {}
impl Action for LoadBinaryData {}
impl Named for LoadBinaryData {}
