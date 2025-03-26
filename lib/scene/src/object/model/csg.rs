use super::{DeserializableRTModel, Hit, RTModel};
use crate::{ImageCache, ImageLoader};

use schemars::JsonSchema;
use serde::Deserialize;
use seui_engine_raytracing_csg_renderer_core::types::rt::Ray;

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeserializableUnion {
    a: Box<DeserializableRTModel>,
    b: Box<DeserializableRTModel>,
}

impl DeserializableUnion {
    pub fn into_rt_model<T: ImageLoader>(
        self,
        image_cache: &mut ImageCache<T>,
    ) -> Box<dyn RTModel + Send + Sync> {
        Box::new(Union {
            a: self.a.into_rt_model(image_cache),
            b: self.b.into_rt_model(image_cache),
        })
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeserializableIntersection {
    a: Box<DeserializableRTModel>,
    b: Box<DeserializableRTModel>,
}

impl DeserializableIntersection {
    pub fn into_rt_model<T: ImageLoader>(
        self,
        image_cache: &mut ImageCache<T>,
    ) -> Box<dyn RTModel + Send + Sync> {
        Box::new(Intersection {
            a: self.a.into_rt_model(image_cache),
            b: self.b.into_rt_model(image_cache),
        })
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DeserializableDifference {
    a: Box<DeserializableRTModel>,
    b: Box<DeserializableRTModel>,
}

impl DeserializableDifference {
    pub fn into_rt_model<T: ImageLoader>(
        self,
        image_cache: &mut ImageCache<T>,
    ) -> Box<dyn RTModel + Send + Sync> {
        Box::new(Difference {
            a: self.a.into_rt_model(image_cache),
            b: self.b.into_rt_model(image_cache),
        })
    }
}

fn remove_duplicate_hits(sorted: &mut Vec<Hit>) {
    let mut result: Vec<Hit> = Vec::new();
    for hit in sorted.drain(..) {
        if let Some(last) = result.last() {
            if last.is_front_face != hit.is_front_face
                && (last.distance - hit.distance).abs() < 1e-6
            {
                result.pop();
                continue;
            }
        }
        result.push(hit);
    }
    *sorted = result;
}

struct Union {
    a: Box<dyn RTModel + Send + Sync>,
    b: Box<dyn RTModel + Send + Sync>,
}

impl RTModel for Union {
    fn test(&self, ray: Ray) -> Vec<Hit> {
        let mut a_hits = self.a.test(ray);
        let mut b_hits = self.b.test(ray);

        if a_hits.is_empty() {
            return b_hits;
        }
        if b_hits.is_empty() {
            return a_hits;
        }

        let mut all_hits = Vec::new();
        all_hits.append(&mut a_hits);
        all_hits.append(&mut b_hits);
        all_hits.sort_by(|h1, h2| h1.distance.partial_cmp(&h2.distance).unwrap());

        remove_duplicate_hits(&mut all_hits);

        let mut stack = 0;
        let mut result = Vec::new();
        for hit in all_hits {
            if hit.is_front_face {
                if stack == 0 {
                    result.push(hit);
                }
                stack += 1;
            } else {
                stack -= 1;
                if stack == 0 {
                    result.push(hit);
                }
            }
        }

        result
    }
}

struct Intersection {
    a: Box<dyn RTModel + Send + Sync>,
    b: Box<dyn RTModel + Send + Sync>,
}

impl RTModel for Intersection {
    fn test(&self, ray: Ray) -> Vec<Hit> {
        let mut a_hits = self.a.test(ray);
        if a_hits.is_empty() {
            return a_hits;
        }

        let mut b_hits = self.b.test(ray);
        if b_hits.is_empty() {
            return b_hits;
        }

        let mut all_hits = Vec::new();
        all_hits.append(&mut a_hits);
        all_hits.append(&mut b_hits);
        all_hits.sort_by(|h1, h2| h1.distance.partial_cmp(&h2.distance).unwrap());

        remove_duplicate_hits(&mut all_hits);

        let mut stack = 0;
        let mut result = Vec::new();
        for hit in all_hits {
            if hit.is_front_face {
                stack += 1;
                if stack == 2 {
                    result.push(hit);
                }
            } else {
                if stack == 2 {
                    result.push(hit);
                }
                stack -= 1;
            }
        }

        result
    }
}

struct Difference {
    a: Box<dyn RTModel + Send + Sync>,
    b: Box<dyn RTModel + Send + Sync>,
}

impl RTModel for Difference {
    fn test(&self, ray: Ray) -> Vec<Hit> {
        let mut a_hits = self.a.test(ray);
        if a_hits.is_empty() {
            return a_hits;
        }

        let mut b_hits = self.b.test(ray);
        if b_hits.is_empty() {
            return a_hits;
        }

        let mut all_hits = Vec::new();
        all_hits.append(&mut a_hits.clone());
        all_hits.append(&mut a_hits);
        all_hits.append(&mut b_hits);
        all_hits.sort_by(|h1, h2| h1.distance.partial_cmp(&h2.distance).unwrap());

        remove_duplicate_hits(&mut all_hits);

        let mut stack = 0;
        let mut is_front_face = false;
        let mut result = Vec::new();
        for hit in all_hits {
            let prev_stack = stack;
            if hit.is_front_face {
                stack += 1;
            } else {
                stack -= 1;
            }
            if prev_stack == 2 || stack == 2 {
                is_front_face = !is_front_face;
                result.push(if is_front_face == hit.is_front_face {
                    hit
                } else {
                    Hit {
                        is_front_face,
                        normal: -hit.normal,
                        ..hit
                    }
                });
            }
        }

        remove_duplicate_hits(&mut result);

        result
    }
}
