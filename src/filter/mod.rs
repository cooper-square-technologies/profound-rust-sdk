mod table;

pub use table::{ALL_FIELDS, Field, Layer, MAX_DEPTH, Op};

use serde::ser::{Serialize, SerializeMap, Serializer};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Str(String),
    List(Vec<String>),
}

impl Serialize for Value {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Str(value) => serializer.serialize_str(value),
            Self::List(value) => value.serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    And(Vec<Filter>),
    Or(Vec<Filter>),
    Not(Box<Filter>),
    Leaf { field: Field, op: Op, value: Option<Value> },
}

impl Serialize for Filter {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::And(nodes) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("and", nodes)?;
                map.end()
            }
            Self::Or(nodes) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("or", nodes)?;
                map.end()
            }
            Self::Not(node) => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("not", node)?;
                map.end()
            }
            Self::Leaf { field, op, value } => {
                let mut map = serializer.serialize_map(Some(if value.is_some() { 3 } else { 2 }))?;
                map.serialize_entry("field", field.name())?;
                map.serialize_entry("op", op.as_str())?;
                if let Some(value) = value {
                    map.serialize_entry("value", value)?;
                }
                map.end()
            }
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum FilterError {
    #[error("Field \"{field}\" does not support op \"{op}\"")]
    UnsupportedOp { field: String, op: String },
    #[error("Filter tree exceeds the maximum nesting depth of {max} (got {depth})")]
    TooDeep { depth: usize, max: usize },
    #[error("Cannot mix prompt-layer and entity-layer fields under \"{kind}\"")]
    MixedLayers { kind: &'static str },
    #[error("\"in\" and \"not_in\" require a non-empty list of values")]
    EmptyList,
    #[error("\"matches\" requires a regex pattern of at least 3 characters")]
    PatternTooShort,
}

impl Filter {
    fn depth(&self) -> usize {
        match self {
            Self::And(nodes) | Self::Or(nodes) => 1 + nodes.iter().map(Self::depth).max().unwrap_or(0),
            Self::Not(node) => 1 + node.depth(),
            Self::Leaf { .. } => 1,
        }
    }

    fn collect_layers(&self, layer: &mut Option<Layer>) -> bool {
        match self {
            Self::And(nodes) | Self::Or(nodes) => nodes.iter().all(|node| node.collect_layers(layer)),
            Self::Not(node) => node.collect_layers(layer),
            Self::Leaf { field, .. } => match layer {
                Some(existing) => *existing == field.layer(),
                None => {
                    *layer = Some(field.layer());
                    true
                }
            },
        }
    }

    fn check_depth(self) -> Result<Self, FilterError> {
        let depth = self.depth();
        if depth > MAX_DEPTH {
            return Err(FilterError::TooDeep { depth, max: MAX_DEPTH });
        }
        Ok(self)
    }

    fn check_single_layer(&self, kind: &'static str) -> Result<(), FilterError> {
        let mut layer = None;
        if self.collect_layers(&mut layer) {
            Ok(())
        } else {
            Err(FilterError::MixedLayers { kind })
        }
    }

    fn leaf(field: Field, op: Op, value: Option<Value>) -> Result<Self, FilterError> {
        if !field.ops().contains(&op) {
            return Err(FilterError::UnsupportedOp {
                field: field.name().to_owned(),
                op: op.as_str().to_owned(),
            });
        }
        Ok(Self::Leaf { field, op, value })
    }

    fn list_leaf(field: Field, op: Op, values: Vec<String>) -> Result<Self, FilterError> {
        if values.is_empty() {
            return Err(FilterError::EmptyList);
        }
        Self::leaf(field, op, Some(Value::List(values)))
    }

    pub fn and(nodes: Vec<Filter>) -> Result<Self, FilterError> {
        if nodes.is_empty() {
            return Err(FilterError::EmptyList);
        }
        Self::And(nodes).check_depth()
    }

    pub fn or(nodes: Vec<Filter>) -> Result<Self, FilterError> {
        if nodes.is_empty() {
            return Err(FilterError::EmptyList);
        }
        let filter = Self::Or(nodes).check_depth()?;
        filter.check_single_layer("or")?;
        Ok(filter)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(node: Filter) -> Result<Self, FilterError> {
        let filter = Self::Not(Box::new(node)).check_depth()?;
        filter.check_single_layer("not")?;
        Ok(filter)
    }

    pub fn equals<S: Into<String>>(field: Field, value: S) -> Result<Self, FilterError> {
        Self::leaf(field, Op::Is, Some(Value::Str(value.into())))
    }

    pub fn not_equals<S: Into<String>>(field: Field, value: S) -> Result<Self, FilterError> {
        Self::leaf(field, Op::NotIs, Some(Value::Str(value.into())))
    }

    pub fn is_in(field: Field, values: Vec<String>) -> Result<Self, FilterError> {
        Self::list_leaf(field, Op::In, values)
    }

    pub fn not_in(field: Field, values: Vec<String>) -> Result<Self, FilterError> {
        Self::list_leaf(field, Op::NotIn, values)
    }

    pub fn contains<S: Into<String>>(field: Field, value: S) -> Result<Self, FilterError> {
        Self::leaf(field, Op::Contains, Some(Value::Str(value.into())))
    }

    pub fn not_contains<S: Into<String>>(field: Field, value: S) -> Result<Self, FilterError> {
        Self::leaf(field, Op::NotContains, Some(Value::Str(value.into())))
    }

    pub fn contains_insensitive<S: Into<String>>(field: Field, value: S) -> Result<Self, FilterError> {
        Self::leaf(field, Op::ContainsCaseInsensitive, Some(Value::Str(value.into())))
    }

    pub fn not_contains_insensitive<S: Into<String>>(field: Field, value: S) -> Result<Self, FilterError> {
        Self::leaf(field, Op::NotContainsCaseInsensitive, Some(Value::Str(value.into())))
    }

    pub fn matches<S: Into<String>>(field: Field, pattern: S) -> Result<Self, FilterError> {
        let pattern = pattern.into();
        if pattern.chars().count() < 3 {
            return Err(FilterError::PatternTooShort);
        }
        Self::leaf(field, Op::Matches, Some(Value::Str(pattern)))
    }

    pub fn exists(field: Field) -> Result<Self, FilterError> {
        Self::leaf(field, Op::Exists, None)
    }

    pub fn to_value(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("filter serialization is infallible")
    }
}

#[cfg(test)]
mod tests {
    use super::{ALL_FIELDS, Field, Filter, FilterError, Layer, MAX_DEPTH, Op};

    #[test]
    fn nested_tree_serializes_to_the_exact_json_tree() {
        let tree = Filter::and(vec![
            Filter::or(vec![
                Filter::equals(Field::Model, "ChatGPT").unwrap(),
                Filter::equals(Field::Model, "Perplexity").unwrap(),
            ])
            .unwrap(),
            Filter::not(Filter::equals(Field::Region, "United States").unwrap()).unwrap(),
        ])
        .unwrap();
        assert_eq!(
            tree.to_value(),
            serde_json::json!({
                "and": [
                    {"or": [
                        {"field": "model", "op": "is", "value": "ChatGPT"},
                        {"field": "model", "op": "is", "value": "Perplexity"}
                    ]},
                    {"not": {"field": "region", "op": "is", "value": "United States"}}
                ]
            })
        );
    }

    #[test]
    fn unsupported_op_is_rejected() {
        assert!(matches!(
            Filter::contains(Field::Theme, "x"),
            Err(FilterError::UnsupportedOp { .. })
        ));
    }

    #[test]
    fn nesting_deeper_than_three_is_rejected() {
        let leaf = Filter::equals(Field::Model, "ChatGPT").unwrap();
        assert!(Filter::and(vec![Filter::or(vec![Filter::not(leaf).unwrap()]).unwrap()]).is_err());
        assert!(
            Filter::or(vec![
                Filter::not(Filter::equals(Field::Model, "ChatGPT").unwrap()).unwrap()
            ])
            .is_ok()
        );
    }

    #[test]
    fn layer_mixing_is_rejected_only_under_or_and_not() {
        assert!(matches!(
            Filter::or(vec![
                Filter::equals(Field::Model, "ChatGPT").unwrap(),
                Filter::equals(Field::Domain, "example.com").unwrap()
            ]),
            Err(FilterError::MixedLayers { kind: "or" })
        ));
        assert!(
            Filter::and(vec![
                Filter::equals(Field::Model, "ChatGPT").unwrap(),
                Filter::equals(Field::Domain, "example.com").unwrap()
            ])
            .is_ok()
        );
    }

    #[test]
    fn op_specific_validations_are_enforced() {
        assert_eq!(Filter::is_in(Field::Model, Vec::new()), Err(FilterError::EmptyList));
        assert_eq!(Filter::matches(Field::Prompt, "ab"), Err(FilterError::PatternTooShort));
        assert_eq!(
            Filter::exists(Field::Tag).unwrap().to_value(),
            serde_json::json!({"field": "tag", "op": "exists"})
        );
    }

    #[test]
    fn generated_table_matches_fixture() {
        let grammar: serde_json::Value =
            serde_json::from_str(include_str!("../../tests/fixtures/filter-grammar.openapi.json")).unwrap();
        assert_eq!(grammar["x-profound-filter-grammar"]["max_depth"], MAX_DEPTH);
        for field in ALL_FIELDS {
            let spec = &grammar["x-profound-filter-grammar"]["fields"][field.name()];
            assert_eq!(
                field.layer(),
                match spec["layer"].as_str().unwrap() {
                    "prompt" => Layer::Prompt,
                    "entity" => Layer::Entity,
                    layer => panic!("unexpected layer {layer}"),
                }
            );
            let expected_ops: Vec<&str> = spec["ops"]
                .as_array()
                .unwrap()
                .iter()
                .map(|op| op.as_str().unwrap())
                .collect();
            let actual_ops: Vec<&str> = field.ops().iter().map(|op: &Op| op.as_str()).collect();
            assert_eq!(actual_ops, expected_ops);
        }
    }
}
