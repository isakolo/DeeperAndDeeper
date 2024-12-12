use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Scene {
    id: usize,
    text: Vec<String>,
    //    choice: Option<(u8, u8)>,
}

pub fn load_scenes() -> Vec<Scene> {
    let json_file_path = std::path::Path::new("../assets/Scenes/testScene.json");

    let file = std::fs::File::open(json_file_path).unwrap();

    let scenes: Vec<Scene> = serde_json::from_reader(file).expect("error while reading or parsing");
    println!("Testing");
    for s in &scenes {
        dbg!(s);
    }
    scenes
}
