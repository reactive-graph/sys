use std::str::FromStr;
use std::sync::LazyLock;

use async_trait::async_trait;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use http::Response;
use http::StatusCode;
use http::request::Request;
use log::debug;
use matchit::Router;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::prelude::*;
use reactive_graph_plugin_api::EntityInstanceManager;
use reactive_graph_plugin_api::HttpBody;
use reactive_graph_plugin_api::WebResourceProvider;
use reactive_graph_plugin_api::prelude::plugin::*;
use serde_json::json;
use strum_macros::AsRefStr;
use strum_macros::Display;
use strum_macros::IntoStaticStr;
use uuid::Uuid;

const CONTEXT_PATH: &str = "binary";

static ID: LazyLock<Uuid> = LazyLock::new(Uuid::new_v4);

#[derive(AsRefStr, IntoStaticStr, Display)]
enum BinaryRequestType {
    Uuid,
    Label,
}

#[derive(AsRefStr, IntoStaticStr, Display, Clone, Debug)]
enum EntityInstanceReference {
    Id(Uuid),
    Label(String),
}

#[derive(Clone, Debug)]
struct PropertyReference {
    pub entity_instance: EntityInstanceReference,
    pub property_name: String,
}

#[derive(Component)]
pub struct BinaryWebResourceProvider {
    #[component(default = "crate::plugin::entity_instance_manager")]
    entity_instance_manager: Arc<dyn EntityInstanceManager + Send + Sync>,
}

impl BinaryWebResourceProvider {
    fn get_property_reference(&self, search: String) -> Option<PropertyReference> {
        let mut matcher = Router::new();
        let _ = matcher.insert("/entities/:uuid/:property_name", BinaryRequestType::Uuid);
        let _ = matcher.insert("/entities/label/*label", BinaryRequestType::Label);
        match matcher.at(search.as_str()) {
            Ok(matched) => match matched.value {
                BinaryRequestType::Uuid => match matched.params.get("uuid") {
                    Some(uuid) => match Uuid::from_str(uuid) {
                        Ok(uuid) => matched.params.get("property_name").map(|property_name| PropertyReference {
                            entity_instance: EntityInstanceReference::Id(uuid),
                            property_name: String::from(property_name),
                        }),
                        Err(_) => None,
                    },
                    None => None,
                },
                BinaryRequestType::Label => matched.params.get("label").map(|label| PropertyReference {
                    entity_instance: EntityInstanceReference::Label(String::from(label)),
                    property_name: String::new(),
                }),
            },
            Err(_) => None,
        }
    }

    fn get_data_url(&self, property_reference: PropertyReference) -> Option<String> {
        match property_reference.entity_instance {
            EntityInstanceReference::Id(id) => self
                .entity_instance_manager
                .get(id)
                .and_then(|entity_instance| entity_instance.as_string(property_reference.property_name))
                .and_then(filter_by_base64_data_url),
            EntityInstanceReference::Label(label) => {
                self.entity_instance_manager
                    .get_by_label_with_params(label.as_str())
                    .and_then(|(entity_instance, params)| {
                        debug!("params {:?}", params);
                        // Prefer path variable "property"
                        if let Some(data_url) = params.get("property").and_then(|property_name| entity_instance.as_string(property_name)) {
                            return Some(data_url);
                        }
                        // Try other path variable
                        params.iter().next().and_then(|(_, property_name)| entity_instance.as_string(property_name))
                    })
            }
        }
    }

    fn set_data_url_binary(&self, property_reference: PropertyReference, bytes: &Vec<u8>) {
        if let EntityInstanceReference::Id(id) = property_reference.entity_instance {
            if let Some(entity_instance) = self.entity_instance_manager.get(id) {
                if let Some(mime_type) = infer::get(bytes) {
                    let data_as_base64 = STANDARD.encode(bytes);
                    let data_url = json!(format!("data:{};base64,{}", mime_type, data_as_base64));
                    entity_instance.set(property_reference.property_name, data_url);
                }
            }
        }
    }

    fn set_data_url_base64(&self, property_reference: PropertyReference, data_url_base64: &String) {
        match property_reference.entity_instance {
            EntityInstanceReference::Id(id) => {
                if let Some(entity_instance) = self.entity_instance_manager.get(id) {
                    debug!("{} {}", id, property_reference.property_name);
                    entity_instance.set(property_reference.property_name, json!(data_url_base64));
                }
            }
            EntityInstanceReference::Label(_) => {}
        }
    }

    fn extract_data_url_payload(&self, data_url: String) -> Option<HttpBody> {
        let mut parts = data_url.splitn(2, ',');
        parts.next();
        match parts.next() {
            Some(part_base64_encoded_data) => match STANDARD.decode(part_base64_encoded_data) {
                Ok(bytes) => Some(HttpBody::Binary(bytes)),
                Err(_) => None,
            },
            None => None,
        }
    }

    fn decode_data_url(&self, data_url: String) -> http::Result<Response<HttpBody>> {
        match self.extract_data_url_payload(data_url) {
            Some(body) => Response::builder().status(StatusCode::OK).body(body),
            None => not_found(),
        }
    }

    fn download(&self, property_reference: PropertyReference) -> http::Result<Response<HttpBody>> {
        match self.get_data_url(property_reference) {
            Some(data_url) => self.decode_data_url(data_url),
            None => not_found(),
        }
    }

    fn upload(&self, property_reference: PropertyReference, request: Request<HttpBody>) -> http::Result<Response<HttpBody>> {
        match request.body() {
            HttpBody::Binary(bytes) => {
                debug!("upload binary");
                self.set_data_url_binary(property_reference.clone(), bytes);
            }
            HttpBody::PlainText(data_url_base64) => {
                debug!("upload data url");
                self.set_data_url_base64(property_reference.clone(), data_url_base64);
            }
            _ => {}
        }
        self.download(property_reference)
    }
}

#[async_trait]
#[component_alias]
impl WebResourceProvider for BinaryWebResourceProvider {
    fn id(&self) -> Uuid {
        *ID
    }

    fn get_context_path(&self) -> String {
        CONTEXT_PATH.to_string()
    }

    async fn handle_web_resource(&self, path: String, request: Request<HttpBody>) -> http::Result<Response<HttpBody>> {
        let uri = request.uri();
        debug!("uri: {uri}");
        debug!("path: {path}");
        let search = format!("/{path}");
        debug!("search: {search}");
        let property_reference = self.get_property_reference(search);
        if property_reference.is_none() {
            return not_found();
        }
        let property_reference = property_reference.unwrap();
        debug!("property_reference: {} {:?}", property_reference.property_name, property_reference.entity_instance);

        let method = request.method().as_str();
        debug!("request: {}", request.method());

        match method {
            "GET" => self.download(property_reference),
            "POST" => self.upload(property_reference, request),
            _ => not_found(),
        }
    }
}

fn not_found() -> http::Result<Response<HttpBody>> {
    Response::builder().status(StatusCode::NOT_FOUND).body(HttpBody::None)
}

fn filter_by_base64_data_url(s: String) -> Option<String> {
    if let Some(prefix) = s.split(',').next() {
        // prefix: data:image/png;base64
        if !prefix.starts_with("data:") || !prefix.ends_with(";base64") {
            return None;
        }
    }
    Some(s)
}
