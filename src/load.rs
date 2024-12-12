use crate::dating_sim::DatingScene;
use serde::Deserialize;

pub fn load_scenes() -> Vec<DatingScene> {
    let json_file_path = std::path::Path::new("../assets/Scenes/testScene.json");

    let file = std::fs::File::open(json_file_path).unwrap();

    let scenes: Vec<DatingScene> =
        serde_json::from_reader(file).expect("error while reading or parsing");
    println!("Testing");
    for s in &scenes {
        dbg!(s);
    }
    scenes
}
