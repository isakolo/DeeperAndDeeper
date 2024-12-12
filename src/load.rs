use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Scene {
    id: usize,
    text: String,
    choice: Option<Vec<String, u8>>,
}

fn main() {
    println!("Hello, world!");
    let _ = load_scene();
}
pub fn load_scenes() -> Vec<Scene> {
    let j = "
        {
            \"id\": \"9\",
            \"text\": \"Hello I am JSON text\",
            \"choice\" none,
        }";

    let parsed: Scene = serde_json::from_str(j).unwrap();
    let instantiated = Scene {
        id: 9,
        text: "Hello I am JSON text",
        choice: None,
    };
    assert_eq!(parsed, instantiated);
}
