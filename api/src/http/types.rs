use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use time::OffsetDateTime;

#[derive(Debug, Clone, sqlx::Type, ScalarType)]
#[scalar(name = "DateTime")]
#[sqlx(transparent)]
pub struct Timestamptz(pub OffsetDateTime);

impl From<OffsetDateTime> for Timestamptz {
    fn from(dt: OffsetDateTime) -> Self {
        Timestamptz(dt)
    }
}

impl AsRef<OffsetDateTime> for Timestamptz {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl Timestamptz {
    pub fn to_string(&self) -> String {
        self.0
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap()
    }
}

#[Scalar]
impl ScalarType for Timestamptz {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(s) = &value {
            OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)
                .map(Timestamptz)
                .map_err(|_| InputValueError::custom("Invalid DateTime format"))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}
