use juniper::ID;

pub fn id_to_i64(id: &ID) -> Result<i64, String> {
    id.parse::<i64>().or(Err("Cannot parse ID".to_string()))
}
