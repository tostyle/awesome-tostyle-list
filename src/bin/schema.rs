use schemars::{gen::SchemaSettings, schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct RepositoryClassifier {
    pub category: String,
    pub id: i32,
    pub name: String,
    // pub topics: Vec<String>,
    pub description: Option<String>,
    pub confidence: f32,
}

fn main() {
    // let schema = schema_for!(RepositoryClassifier);

    let settings = SchemaSettings::draft07().with(|s| {
        s.option_nullable = false;
        s.option_add_null_type = true;
    });
    let generator = settings.into_generator();
    let schema = generator.into_root_schema_for::<RepositoryClassifier>();
    let json_schema = json!(schema);
    println!("{}", serde_json::to_string_pretty(&json_schema).unwrap());
}
