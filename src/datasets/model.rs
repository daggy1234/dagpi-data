use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YoMamaJoke {
    pub description: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YoDataset {
    pub data: Vec<YoMamaJoke>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PickupLine {
    pub category: String,
    pub joke: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PickupLines {
    pub data: Vec<PickupLine>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct BasicMon {
    pub abilities: Vec<String>,
    pub ascii: String,
    pub height: f32,
    pub id: f32,
    pub link: String,
    pub name: String,
    #[allow(non_snake_case)]
    pub Type: Vec<String>,
    pub weight: f32,
}

#[derive(Serialize, Deserialize)]
pub struct MonVec {
    pub list: Vec<BasicMon>,
}

#[derive(Serialize, Deserialize)]
pub struct WaifuData {
    pub data: Vec<JsonValue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Joke {
    pub id: String,
    pub joke: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Jokes {
    pub data: Vec<Joke>,
}

pub struct Roasts {
    pub list: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Logos {
    pub data: Vec<JsonValue>,
}

pub struct Facts {
    pub list: Vec<String>,
}
pub struct EightBall {
    pub list: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headline {
    pub text: String,
    pub fake: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Headlines {
    pub headlines: Vec<Headline>,
}
