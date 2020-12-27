use googapis::google::datastore::v1::{
    gql_query_parameter::ParameterType,
    key::{path_element::IdType, PathElement},
    GqlQueryParameter, Value,
};
use juniper::ID;

use crate::graphql::utils::id_to_i64;

pub fn value_to_gql_param(value: &Value) -> GqlQueryParameter {
    GqlQueryParameter {
        parameter_type: Some(ParameterType::Value(value.clone())),
    }
}

pub fn normalize_path(path: &PathToRef) -> Vec<PathElement> {
    path.iter()
        .map(|(kind, id)| PathElement {
            id_type: match &id {
                KeyId::Id(id) => {
                    if let Ok(id) = id_to_i64(id) {
                        Some(IdType::Id(id))
                    } else {
                        None
                    }
                }
                KeyId::Str(name) => Some(IdType::Name(name.to_string())),
                _ => None,
            },
            kind: kind.0.to_string(),
        })
        .collect()
}

pub type PathToRef<'a> = [(KeyKind<'a>, KeyId<'a>)];

pub struct KeyKind<'a>(pub &'a str);
pub enum KeyId<'a> {
    Str(&'a str),
    Id(&'a ID),
    None,
}
