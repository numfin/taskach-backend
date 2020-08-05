// use juniper::{
//     parser::{ParseError, ScalarToken, Token},
//     InputValue, ParseScalarResult, Value,
// };

// juniper::graphql_scalar!(u32 as "UInt" where Scalar = <S>{
//     resolve(&self) -> Value {
//         Value::scalar(*self)
//     }

//     from_input_value(v: &InputValue) -> Option<u32> {
//         match *v {
//             InputValue::Scalar(ref i) => i.as_int(),
//             _ => None,
//         }
//     }

//     from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
//         if let ScalarToken::Int(v) = value {
//             v.parse()
//              .map_err(|_| ParseError::UnexpectedToken(Token::Scalar(value)))
//              .map(|s: i32| s.into())
//         } else {
//             Err(ParseError::UnexpectedToken(Token::Scalar(value)))
//         }
//     }
// });

// #[crate::graphql_scalar(name = "UInt")]
// impl<S> GraphQLScalar for u32
// where
//     S: ScalarValue,
// {
//     fn resolve(&self) -> Value {
//         Value::scalar(*self)
//     }

//     fn from_input_value(v: &InputValue) -> Option<u32> {
//         match *v {
//             InputValue::Scalar(ref i) => i.as_int(),
//             _ => None,
//         }
//     }

//     fn from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
//         if let ScalarToken::Int(v) = value {
//             v.parse()
//                 .map_err(|_| ParseError::UnexpectedToken(Token::Scalar(value)))
//                 .map(|s: u32| s.into())
//         } else {
//             Err(ParseError::UnexpectedToken(Token::Scalar(value)))
//         }
//     }
// }
