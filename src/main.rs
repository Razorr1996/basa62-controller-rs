use futures::{pin_mut, TryStreamExt};
use k8s_openapi::{
    api::core::v1::Node,
    apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition,
};
use kube::{
    api::{ListParams, Patch, PatchParams},
    runtime::{conditions, wait::await_condition, watcher, WatchStreamExt},
    Api, Client, CustomResource, CustomResourceExt, ResourceExt,
};
use log::info;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(group = "basa62.com", version = "v1", kind = "Topology", namespaced)]
#[kube(status = "TopologyStatus")]
struct TopologySpec {
    pub name: String,
    pub nodes: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
struct TopologyStatus {
    pub is_ok: bool,
}

const CRD_NAME: &str = "topologies.basa62.com";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let client = Client::try_default().await?;

    let top_apply = PatchParams::apply("topology_apply").force();
    let crd_client: Api<CustomResourceDefinition> = Api::all(client.clone());

    crd_client
        .patch(CRD_NAME, &top_apply, &Patch::Apply(Topology::crd()))
        .await?;

    info!(
        "Creating our CRD: {}",
        serde_yaml::to_string(&Topology::crd())?
    );

    info!("Waiting for the api-server to accept the CRD");
    let establish = await_condition(crd_client, CRD_NAME, conditions::is_crd_established());
    let _ = tokio::time::timeout(std::time::Duration::from_secs(10), establish).await?;

    // Let's get the current node topology
    let nodes: Api<Node> = Api::all(client.clone());
    let topologies: Api<Topology> = Api::default_namespaced(client.clone());

    let spec = create_spec(nodes).await;

    let applied1 = topologies
        .patch(
            "default",
            &top_apply,
            &Patch::Apply(&Topology::new("default", spec)),
        )
        .await?;

    info!("Applied 1 {}: {:?}", applied1.name_any(), applied1.spec);

    // Watch the topology resource
    let obs = watcher(topologies, watcher::Config::default()).applied_objects();
    pin_mut!(obs);

    while let Some(topology) = obs.try_next().await? {
        match topology {
            Topology { .. } => {
                let nodes: Api<Node> = Api::all(client.clone());
                let spec = create_spec(nodes.clone()).await;
                let topologys: Api<Topology> = Api::default_namespaced(client.clone());

                let applied2 = topologys
                    .patch(
                        "default",
                        &top_apply,
                        &Patch::Apply(&Topology::new("default", spec)),
                    )
                    .await?;

                info!("Applied 2 {}: {:?}", applied2.name_any(), applied2.spec);
            }
        }
    }

    anyhow::Ok(())
}

async fn create_spec(nodes: Api<Node>) -> TopologySpec {
    let node_list = nodes.list(&ListParams::default()).await.unwrap();

    let node_names = node_list
        .into_iter()
        .map(|n| n.metadata.name.unwrap())
        .collect();

    TopologySpec {
        name: "default".to_string(),
        nodes: node_names,
    }
}
