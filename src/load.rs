struct Scene {
    id: usize,
    text: String,
    choice: Vec<String, u8>,
}

fn main() {
    println!("Hello, world!");
}
pub fn load_scenes() -> Vec<Scene> {
    let parsed = json::parse(
        r#"
{
    "": 200,
    "success": true,
    "payload": {
        "features": [
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
}

"#,
    )
    .unwrap();

    let instantiated = object! {
        // quotes on keys are optional
        "code": 200,
        success: true,
        payload: {
            features: [
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    };
    assert_eq!(parsed, instantiated);
}
