use schemars::schema_for;
use seui_engine_raytracing_csg_renderer_scene::DeserializableScene;
use std::fs::File;
use std::io::Write;

fn generate_json_schema() {
    let schema = schema_for!(DeserializableScene);
    let schema_json = serde_json::to_string_pretty(&schema).unwrap();

    let mut file = File::create("schema.json").expect("Failed to create schema file");
    file.write_all(schema_json.as_bytes())
        .expect("Failed to write schema");
}

fn main() {
    generate_json_schema();
}
