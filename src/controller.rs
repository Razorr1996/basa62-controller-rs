use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(kind = "HelloApp", group = "basa62.com", version = "v1", namespaced)]
#[kube(status = "HelloAppStatus")]
pub struct HelloAppSpec {
    pub image: String,
    pub replicas: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct HelloAppStatus {
    replicas: u32,
}
