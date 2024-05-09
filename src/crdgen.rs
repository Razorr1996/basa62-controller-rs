use kube::CustomResourceExt;

fn main() {
    println!(
        "{}",
        serde_yaml::to_string(&basa62_controller::HelloApp::crd()).unwrap()
    )
}
