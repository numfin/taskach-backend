use cuid::cuid;
use googapis::google::datastore::v1::{
    gql_query_parameter::ParameterType,
    key::{path_element::IdType, PathElement},
    GqlQueryParameter, Value,
};
use juniper::ID;

use crate::graphql::utils::id_to_i64;

/// Convert Value to Parameter (for using in Queries)
pub fn value_to_gql_param(value: &Value) -> GqlQueryParameter {
    GqlQueryParameter {
        parameter_type: Some(ParameterType::Value(value.clone())),
    }
}

/// Convert from `(KeyKind, KeyId)[]` to `PathElement[]`
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
                KeyId::Cuid(id) => Some(IdType::Name(id.to_string())),
            },
            kind: kind.0.to_string(),
        })
        .collect()
}

/// `[(Kind, Id)]` pairs
pub type PathToRef<'a> = [(KeyKind<'a>, KeyId<'a>)];

/// Kind of entity (Table)
#[derive(Debug)]
pub struct KeyKind<'a>(pub &'a str);

/// Id of entity (cuid, i64, etc)
#[derive(Debug)]
pub enum KeyId<'a> {
    Id(&'a ID),
    Cuid(&'a String),
}

/// Generate CUID
pub fn gen_cuid() -> Result<String, String> {
    cuid().map_err(|err| {
        println!("{:?}", err.to_string());
        format!("Cannot create cuid")
    })
}
