use kube::CustomResource;
use num_traits::One;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    kind = "HelloApp",
    group = "basa62.com",
    version = "v1",
    shortname = "ha",
    namespaced
)]
#[kube(status = "HelloAppStatus")]
#[kube(printcolumn = r#"{
  "name": "Image",
  "type": "string",
  "description": "Image name[:tag]",
  "jsonPath": ".spec.image"
}"#)]
#[kube(printcolumn = r#"{
  "name": "Replicas",
  "type": "string",
  "description": "Count of replicas",
  "jsonPath": ".spec.replicas"
}"#)]
pub struct HelloAppSpec {
    pub image: String,
    #[serde(default = "u32::one")]
    pub replicas: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct HelloAppStatus {
    replicas: u32,
}
