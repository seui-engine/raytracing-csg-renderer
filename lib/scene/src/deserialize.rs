use glam::Vec3;
use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::Deserializer;
use seui_engine_raytracing_csg_renderer_core::types::math::{Direction, Move, Position};
use seui_engine_raytracing_csg_renderer_types::{HDRColor, LDRColor};
use std::fmt;

use crate::json_schema::Scale;

pub fn deserialize_hdr_color<'de, D>(deserializer: D) -> Result<HDRColor, D::Error>
where
    D: Deserializer<'de>,
{
    struct Vec3Visitor;

    impl<'de> Visitor<'de> for Vec3Visitor {
        type Value = HDRColor;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a struct {r, g, b} or an array [r, g, b]")
        }

        // Deserialize from { r, g, b }
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut r = None;
            let mut g = None;
            let mut b = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "r" => r = Some(map.next_value()?),
                    "g" => g = Some(map.next_value()?),
                    "b" => b = Some(map.next_value()?),
                    _ => return Err(de::Error::unknown_field(&key, &["r", "g", "b"])),
                }
            }

            let r = r.ok_or_else(|| de::Error::missing_field("r"))?;
            let g = g.ok_or_else(|| de::Error::missing_field("g"))?;
            let b = b.ok_or_else(|| de::Error::missing_field("b"))?;

            Ok(HDRColor { r, g, b })
        }

        // Deserialize from [r, g, b]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let r = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
            let g = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(1, &self))?;
            let b = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(2, &self))?;

            Ok(HDRColor { r, g, b })
        }
    }

    deserializer.deserialize_any(Vec3Visitor)
}

pub fn deserialize_ldr_color<'de, D>(deserializer: D) -> Result<LDRColor, D::Error>
where
    D: Deserializer<'de>,
{
    struct Vec3Visitor;

    impl<'de> Visitor<'de> for Vec3Visitor {
        type Value = LDRColor;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a struct {r, g, b} or an array [r, g, b]")
        }

        // Deserialize from { r, g, b }
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut r = None;
            let mut g = None;
            let mut b = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "r" => r = Some(map.next_value()?),
                    "g" => g = Some(map.next_value()?),
                    "b" => b = Some(map.next_value()?),
                    _ => return Err(de::Error::unknown_field(&key, &["r", "g", "b"])),
                }
            }

            let r = r.ok_or_else(|| de::Error::missing_field("r"))?;
            let g = g.ok_or_else(|| de::Error::missing_field("g"))?;
            let b = b.ok_or_else(|| de::Error::missing_field("b"))?;

            Ok(LDRColor::new(r, g, b))
        }

        // Deserialize from [r, g, b]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let r = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
            let g = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(1, &self))?;
            let b = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(2, &self))?;

            Ok(LDRColor::new(r, g, b))
        }
    }

    deserializer.deserialize_any(Vec3Visitor)
}

pub fn deserialize_position<'de, D>(deserializer: D) -> Result<Position, D::Error>
where
    D: Deserializer<'de>,
{
    struct Vec3Visitor;

    impl<'de> Visitor<'de> for Vec3Visitor {
        type Value = Position;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a struct {x, y, z} or an array [x, y, z]")
        }

        // Deserialize from { x, y, z }
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut x = None;
            let mut y = None;
            let mut z = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "x" => x = Some(map.next_value()?),
                    "y" => y = Some(map.next_value()?),
                    "z" => z = Some(map.next_value()?),
                    _ => return Err(de::Error::unknown_field(&key, &["x", "y", "z"])),
                }
            }

            let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
            let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
            let z = z.ok_or_else(|| de::Error::missing_field("z"))?;

            Ok(Position::new(Vec3::new(x, y, z)))
        }

        // Deserialize from [x, y, z]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let x = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
            let y = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(1, &self))?;
            let z = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(2, &self))?;

            Ok(Position::new(Vec3::new(x, y, z)))
        }
    }

    deserializer.deserialize_any(Vec3Visitor)
}

pub fn deserialize_direction<'de, D>(deserializer: D) -> Result<Direction, D::Error>
where
    D: Deserializer<'de>,
{
    struct Vec3Visitor;

    impl<'de> Visitor<'de> for Vec3Visitor {
        type Value = Direction;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a struct {x, y, z} or an array [x, y, z]")
        }

        // Deserialize from { x, y, z }
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut x = None;
            let mut y = None;
            let mut z = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "x" => x = Some(map.next_value()?),
                    "y" => y = Some(map.next_value()?),
                    "z" => z = Some(map.next_value()?),
                    _ => return Err(de::Error::unknown_field(&key, &["x", "y", "z"])),
                }
            }

            let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
            let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
            let z = z.ok_or_else(|| de::Error::missing_field("z"))?;

            Ok(Direction::new(Vec3::new(x, y, z)))
        }

        // Deserialize from [x, y, z]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let x = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
            let y = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(1, &self))?;
            let z = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(2, &self))?;

            Ok(Direction::new(Vec3::new(x, y, z)))
        }
    }

    deserializer.deserialize_any(Vec3Visitor)
}

pub fn deserialize_move<'de, D>(deserializer: D) -> Result<Move, D::Error>
where
    D: Deserializer<'de>,
{
    struct Vec3Visitor;

    impl<'de> Visitor<'de> for Vec3Visitor {
        type Value = Move;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a struct {x, y, z} or an array [x, y, z]")
        }

        // Deserialize from { x, y, z }
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut x = None;
            let mut y = None;
            let mut z = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "x" => x = Some(map.next_value()?),
                    "y" => y = Some(map.next_value()?),
                    "z" => z = Some(map.next_value()?),
                    _ => return Err(de::Error::unknown_field(&key, &["x", "y", "z"])),
                }
            }

            let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
            let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
            let z = z.ok_or_else(|| de::Error::missing_field("z"))?;

            Ok(Move::new(Vec3::new(x, y, z)))
        }

        // Deserialize from [x, y, z]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let x = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
            let y = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(1, &self))?;
            let z = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(2, &self))?;

            Ok(Move::new(Vec3::new(x, y, z)))
        }
    }

    deserializer.deserialize_any(Vec3Visitor)
}

pub fn deserialize_scale<'de, D>(deserializer: D) -> Result<Scale, D::Error>
where
    D: Deserializer<'de>,
{
    struct Vec3Visitor;

    impl<'de> Visitor<'de> for Vec3Visitor {
        type Value = Scale;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a struct {x, y, z} or an array [x, y, z]")
        }

        // Deserialize from { x, y, z }
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut x = None;
            let mut y = None;
            let mut z = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "x" => x = Some(map.next_value()?),
                    "y" => y = Some(map.next_value()?),
                    "z" => z = Some(map.next_value()?),
                    _ => return Err(de::Error::unknown_field(&key, &["x", "y", "z"])),
                }
            }

            let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
            let y = y.ok_or_else(|| de::Error::missing_field("y"))?;
            let z = z.ok_or_else(|| de::Error::missing_field("z"))?;

            Ok(Scale { x, y, z })
        }

        // Deserialize from [x, y, z]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let x = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
            let y = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(1, &self))?;
            let z = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(2, &self))?;

            Ok(Scale { x, y, z })
        }
    }

    deserializer.deserialize_any(Vec3Visitor)
}
