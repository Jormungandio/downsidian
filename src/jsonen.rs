use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub name: String,
    pub description: String,
    pub test: String,
    pub lesson: String,
    pub related: Vec<String>
}

/* 

New для поиска темы и уровня, возвращает content 
 */
pub fn new(theme: String, level: String) -> Content {
    let file_jsone = fs::read_to_string("/home/zahar/gptskill.json").unwrap();
    
    let json_data: Value = serde_json::from_str(&file_jsone).unwrap();

    let pc = json_data.get(&theme).unwrap().as_object().unwrap().get(&level).unwrap().as_object().unwrap();

    let content: Content = serde_json::from_value(Value::Object(pc.clone())).unwrap();

    content
}

pub fn all() -> Vec<String>{
    let json_data = fs::read_to_string("/home/zahar/gptskill.json").unwrap();
    let parsed_data: Value = serde_json::from_str(&json_data).unwrap();
    let mut skills: Vec<String> = Vec::new();

    for key in parsed_data.as_object().unwrap().keys() {
        skills.push(key.to_string());
    }

    skills
}