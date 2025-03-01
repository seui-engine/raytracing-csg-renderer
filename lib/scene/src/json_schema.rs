use std::collections::BTreeSet;

use schemars::{
    schema::{InstanceType, Metadata, ObjectValidation, Schema, SchemaObject},
    JsonSchema, SchemaGenerator,
};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionSchema {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl JsonSchema for PositionSchema {
    fn schema_name() -> String {
        "Position".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            metadata: Some(Box::new(Metadata {
                title: Some("Position".to_string()),
                description: Some(
                    "A 3D position, either as an object `{x, y, z}` or a tuple `[x, y, z]`."
                        .to_string(),
                ),
                ..Default::default()
            })),
            instance_type: None, // We define multiple formats below
            subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                one_of: Some(vec![
                    // Object format: { x: f32, y: f32, z: f32 }
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Object.into()),
                        object: Some(Box::new(ObjectValidation {
                            properties: [
                                ("x".to_string(), gen.subschema_for::<f32>()),
                                ("y".to_string(), gen.subschema_for::<f32>()),
                                ("z".to_string(), gen.subschema_for::<f32>()),
                            ]
                            .into_iter()
                            .collect(),
                            required: BTreeSet::from([
                                "x".to_string(),
                                "y".to_string(),
                                "z".to_string(),
                            ]),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    // Tuple format: [x, y, z]
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(schemars::schema::ArrayValidation {
                            items: Some(gen.subschema_for::<f32>().into()),
                            min_items: Some(3),
                            max_items: Some(3),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scale {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl JsonSchema for Scale {
    fn schema_name() -> String {
        "Scale".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            metadata: Some(Box::new(Metadata {
                title: Some("Scale".to_string()),
                description: Some(
                    "A 3D scale, either as an object `{x, y, z}` or a tuple `[x, y, z]`."
                        .to_string(),
                ),
                ..Default::default()
            })),
            instance_type: None, // We define multiple formats below
            subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                one_of: Some(vec![
                    // Object format: { x: f32, y: f32, z: f32 }
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Object.into()),
                        object: Some(Box::new(ObjectValidation {
                            properties: [
                                ("x".to_string(), gen.subschema_for::<f32>()),
                                ("y".to_string(), gen.subschema_for::<f32>()),
                                ("z".to_string(), gen.subschema_for::<f32>()),
                            ]
                            .into_iter()
                            .collect(),
                            required: BTreeSet::from([
                                "x".to_string(),
                                "y".to_string(),
                                "z".to_string(),
                            ]),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    // Tuple format: [x, y, z]
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(schemars::schema::ArrayValidation {
                            items: Some(gen.subschema_for::<f32>().into()),
                            min_items: Some(3),
                            max_items: Some(3),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectionSchema {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl JsonSchema for DirectionSchema {
    fn schema_name() -> String {
        "Direction".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            metadata: Some(Box::new(Metadata {
                title: Some("Direction".to_string()),
                description: Some(
                    "A 3D normal, either as an object `{x, y, z}` or a tuple `[x, y, z]`."
                        .to_string(),
                ),
                ..Default::default()
            })),
            instance_type: None, // We define multiple formats below
            subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                one_of: Some(vec![
                    // Object format: { x: f32, y: f32, z: f32 }
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Object.into()),
                        object: Some(Box::new(ObjectValidation {
                            properties: [
                                ("x".to_string(), gen.subschema_for::<f32>()),
                                ("y".to_string(), gen.subschema_for::<f32>()),
                                ("z".to_string(), gen.subschema_for::<f32>()),
                            ]
                            .into_iter()
                            .collect(),
                            required: BTreeSet::from([
                                "x".to_string(),
                                "y".to_string(),
                                "z".to_string(),
                            ]),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    // Tuple format: [x, y, z]
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(schemars::schema::ArrayValidation {
                            items: Some(gen.subschema_for::<f32>().into()),
                            min_items: Some(3),
                            max_items: Some(3),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveSchema {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl JsonSchema for MoveSchema {
    fn schema_name() -> String {
        "Move".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            metadata: Some(Box::new(Metadata {
                title: Some("Move".to_string()),
                description: Some(
                    "A 3D vector, either as an object `{x, y, z}` or a tuple `[x, y, z]`."
                        .to_string(),
                ),
                ..Default::default()
            })),
            instance_type: None, // We define multiple formats below
            subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                one_of: Some(vec![
                    // Object format: { x: f32, y: f32, z: f32 }
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Object.into()),
                        object: Some(Box::new(ObjectValidation {
                            properties: [
                                ("x".to_string(), gen.subschema_for::<f32>()),
                                ("y".to_string(), gen.subschema_for::<f32>()),
                                ("z".to_string(), gen.subschema_for::<f32>()),
                            ]
                            .into_iter()
                            .collect(),
                            required: BTreeSet::from([
                                "x".to_string(),
                                "y".to_string(),
                                "z".to_string(),
                            ]),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    // Tuple format: [x, y, z]
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(schemars::schema::ArrayValidation {
                            items: Some(gen.subschema_for::<f32>().into()),
                            min_items: Some(3),
                            max_items: Some(3),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HDRColorSchema {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl JsonSchema for HDRColorSchema {
    fn schema_name() -> String {
        "HDRColor".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            metadata: Some(Box::new(Metadata {
                title: Some("HDRColor".to_string()),
                description: Some(
                    "A HDR color, either as an object `{r, g, b}` or a tuple `[r, g, b]`."
                        .to_string(),
                ),
                ..Default::default()
            })),
            instance_type: None, // We define multiple formats below
            subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                one_of: Some(vec![
                    // Object format: { x: f32, y: f32, z: f32 }
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Object.into()),
                        object: Some(Box::new(ObjectValidation {
                            properties: [
                                ("r".to_string(), gen.subschema_for::<f32>()),
                                ("g".to_string(), gen.subschema_for::<f32>()),
                                ("b".to_string(), gen.subschema_for::<f32>()),
                            ]
                            .into_iter()
                            .collect(),
                            required: BTreeSet::from([
                                "r".to_string(),
                                "g".to_string(),
                                "b".to_string(),
                            ]),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    // Tuple format: [x, y, z]
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(schemars::schema::ArrayValidation {
                            items: Some(gen.subschema_for::<f32>().into()),
                            min_items: Some(3),
                            max_items: Some(3),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LDRColorSchema {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl JsonSchema for LDRColorSchema {
    fn schema_name() -> String {
        "LDRColor".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            metadata: Some(Box::new(Metadata {
                title: Some("LDRColor".to_string()),
                description: Some(
                    "A LDR color, either as an object `{r, g, b}` or a tuple `[r, g, b]`."
                        .to_string(),
                ),
                ..Default::default()
            })),
            instance_type: None, // We define multiple formats below
            subschemas: Some(Box::new(schemars::schema::SubschemaValidation {
                one_of: Some(vec![
                    // Object format: { x: f32, y: f32, z: f32 }
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Object.into()),
                        object: Some(Box::new(ObjectValidation {
                            properties: [
                                ("r".to_string(), gen.subschema_for::<f32>()),
                                ("g".to_string(), gen.subschema_for::<f32>()),
                                ("b".to_string(), gen.subschema_for::<f32>()),
                            ]
                            .into_iter()
                            .collect(),
                            required: BTreeSet::from([
                                "r".to_string(),
                                "g".to_string(),
                                "b".to_string(),
                            ]),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    // Tuple format: [x, y, z]
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(schemars::schema::ArrayValidation {
                            items: Some(gen.subschema_for::<f32>().into()),
                            min_items: Some(3),
                            max_items: Some(3),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
