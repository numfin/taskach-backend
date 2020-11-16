#[allow(unused_imports)]
use juniper::{InputValue, ParseScalarResult, ParseScalarValue, ScalarValue, Value};

pub struct PositiveNumber(u32);
impl std::str::FromStr for PositiveNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        s.parse::<u32>()
            .map(|v| PositiveNumber(v))
            .map_err(|_| String::from("This is not UInt"))
    }
}

#[juniper::graphql_scalar(description = "Positive integer")]
impl<S> GraphQLScalar for PositiveNumber
where
    S: ScalarValue,
{
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value {
        Value::scalar(self.0.to_string())
    }

    // Define how to parse a primitive type into your custom scalar.
    fn from_input_value(v: &InputValue) -> Option<PositiveNumber> {
        v.as_scalar_value()
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
    }

    // Define how to parse a string value.
    fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
}
