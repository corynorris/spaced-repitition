use async_graphql::{InputValueError, InputValueResult, ScalarType, Value};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct Timestamptz(pub OffsetDateTime);

// Keep existing From impl
impl From<OffsetDateTime> for Timestamptz {
    fn from(dt: OffsetDateTime) -> Self {
        Timestamptz(dt)
    }
}

// Keep existing AsRef impl
impl AsRef<OffsetDateTime> for Timestamptz {
    fn as_ref(&self) -> &OffsetDateTime {
        &self.0
    }
}

// Implement equality (helpful for testing)
impl PartialEq for Timestamptz {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Timestamptz {}

// Implement Serialize for standard JSON serialization
impl Serialize for Timestamptz {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0
            .format(&Rfc3339)
            .map_err(serde::ser::Error::custom)
            .and_then(|formatted| serializer.serialize_str(&formatted))
    }
}

// Implement Deserialize for standard JSON deserialization
impl<'de> Deserialize<'de> for Timestamptz {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimestampVisitor;

        impl<'de> serde::de::Visitor<'de> for TimestampVisitor {
            type Value = Timestamptz;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("an RFC3339 formatted datetime string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                OffsetDateTime::parse(value, &Rfc3339)
                    .map(Timestamptz)
                    .map_err(|e| E::custom(format!("invalid timestamp format: {}", e)))
            }
        }

        deserializer.deserialize_str(TimestampVisitor)
    }
}

// Implement ScalarType for GraphQL integration
#[async_graphql::Scalar(name = "DateTime")]
impl ScalarType for Timestamptz {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(s) = &value {
            OffsetDateTime::parse(s, &Rfc3339)
                .map(Timestamptz)
                .map_err(|_| InputValueError::custom("Invalid DateTime format"))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.format(&Rfc3339).unwrap_or_else(|_| String::from("")))
    }
}
