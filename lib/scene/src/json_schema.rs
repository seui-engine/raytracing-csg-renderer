use std::collections::BTreeSet;

use schemars::{
    schema::{
        ArrayValidation, InstanceType, Metadata, NumberValidation, ObjectValidation, Schema,
        SchemaObject, SubschemaValidation,
    },
    JsonSchema, SchemaGenerator,
};

pub struct PositionSchema {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(vec![
                    // Object format: { x: f64, y: f64, z: f64 }
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Object.into()),
                        object: Some(Box::new(ObjectValidation {
                            properties: [
                                ("x".to_string(), gen.subschema_for::<f64>()),
                                ("y".to_string(), gen.subschema_for::<f64>()),
                                ("z".to_string(), gen.subschema_for::<f64>()),
                            ]
                            .into_iter()
                            .collect(),
                            required: BTreeSet::from([
                                "x".to_string(),
                                "y".to_string(),
                                "z".to_string(),
                            ]),
                            additional_properties: Some(Box::new(Schema::Bool(false))),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    // Tuple format: [x, y, z]
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(ArrayValidation {
                            items: Some(gen.subschema_for::<f64>().into()),
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

#[derive(Clone, Copy, Debug)]
pub struct Scale {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Scale {
    fn default() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

impl JsonSchema for Scale {
    fn schema_name() -> String {
        "Scale".to_string()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
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
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(vec![
                    // Object format: { x: f64, y: f64, z: f64 }
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Object.into()),
                        object: Some(Box::new(ObjectValidation {
                            properties: [
                                ("x".to_string(), float_nonnegative()),
                                ("y".to_string(), float_nonnegative()),
                                ("z".to_string(), float_nonnegative()),
                            ]
                            .into_iter()
                            .collect(),
                            required: BTreeSet::from([
                                "x".to_string(),
                                "y".to_string(),
                                "z".to_string(),
                            ]),
                            additional_properties: Some(Box::new(Schema::Bool(false))),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    // Tuple format: [x, y, z]
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(ArrayValidation {
                            items: Some(float_nonnegative().into()),
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

pub struct DirectionSchema {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(vec![
                    // Object format: { x: f64, y: f64, z: f64 }
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Object.into()),
                        object: Some(Box::new(ObjectValidation {
                            properties: [
                                ("x".to_string(), gen.subschema_for::<f64>()),
                                ("y".to_string(), gen.subschema_for::<f64>()),
                                ("z".to_string(), gen.subschema_for::<f64>()),
                            ]
                            .into_iter()
                            .collect(),
                            required: BTreeSet::from([
                                "x".to_string(),
                                "y".to_string(),
                                "z".to_string(),
                            ]),
                            additional_properties: Some(Box::new(Schema::Bool(false))),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    // Tuple format: [x, y, z]
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(ArrayValidation {
                            items: Some(gen.subschema_for::<f64>().into()),
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

pub struct MoveSchema {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(vec![
                    // Object format: { x: f64, y: f64, z: f64 }
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Object.into()),
                        object: Some(Box::new(ObjectValidation {
                            properties: [
                                ("x".to_string(), gen.subschema_for::<f64>()),
                                ("y".to_string(), gen.subschema_for::<f64>()),
                                ("z".to_string(), gen.subschema_for::<f64>()),
                            ]
                            .into_iter()
                            .collect(),
                            required: BTreeSet::from([
                                "x".to_string(),
                                "y".to_string(),
                                "z".to_string(),
                            ]),
                            additional_properties: Some(Box::new(Schema::Bool(false))),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    // Tuple format: [x, y, z]
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(ArrayValidation {
                            items: Some(gen.subschema_for::<f64>().into()),
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

pub struct HDRColorSchema {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl JsonSchema for HDRColorSchema {
    fn schema_name() -> String {
        "HDRColor".to_string()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
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
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(vec![
                    // Object format: { x: f64, y: f64, z: f64 }
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Object.into()),
                        object: Some(Box::new(ObjectValidation {
                            properties: [
                                ("r".to_string(), float_nonnegative()),
                                ("g".to_string(), float_nonnegative()),
                                ("b".to_string(), float_nonnegative()),
                            ]
                            .into_iter()
                            .collect(),
                            required: BTreeSet::from([
                                "r".to_string(),
                                "g".to_string(),
                                "b".to_string(),
                            ]),
                            additional_properties: Some(Box::new(Schema::Bool(false))),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    // Tuple format: [x, y, z]
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(ArrayValidation {
                            items: Some(float_nonnegative().into()),
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

pub struct LDRColorSchema {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl JsonSchema for LDRColorSchema {
    fn schema_name() -> String {
        "LDRColor".to_string()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
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
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(vec![
                    // Object format: { x: f64, y: f64, z: f64 }
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Object.into()),
                        object: Some(Box::new(ObjectValidation {
                            properties: [
                                ("r".to_string(), float_ldr()),
                                ("g".to_string(), float_ldr()),
                                ("b".to_string(), float_ldr()),
                            ]
                            .into_iter()
                            .collect(),
                            required: BTreeSet::from([
                                "r".to_string(),
                                "g".to_string(),
                                "b".to_string(),
                            ]),
                            additional_properties: Some(Box::new(Schema::Bool(false))),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    // Tuple format: [x, y, z]
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(ArrayValidation {
                            items: Some(float_ldr().into()),
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

fn float_nonnegative() -> Schema {
    Schema::Object(SchemaObject {
        instance_type: Some(InstanceType::Number.into()),
        format: Some("float".to_string()),
        number: Some(Box::new(NumberValidation {
            minimum: Some(0.0),
            ..Default::default()
        })),
        ..Default::default()
    })
}

fn float_ldr() -> Schema {
    Schema::Object(SchemaObject {
        instance_type: Some(InstanceType::Number.into()),
        format: Some("float".to_string()),
        number: Some(Box::new(NumberValidation {
            minimum: Some(0.0),
            maximum: Some(1.0),
            ..Default::default()
        })),
        ..Default::default()
    })
}
