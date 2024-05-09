use basa62_controller::{HelloApp, HelloAppSpec};

fn main() {
    let app = HelloApp::new("example", HelloAppSpec {
        image: "nginx:latest".into(),
        replicas: 1,
    });

    print!("{}", serde_yaml::to_string(&app).unwrap())
}
