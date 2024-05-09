use kube::CustomResourceExt;

fn main() {
    print!(
        "{}",
        serde_yaml::to_string(&basa62_controller::HelloApp::crd()).unwrap()
    )
}
