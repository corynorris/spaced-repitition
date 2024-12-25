use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// A wrapper type for RFC3339 timestamp serialization/deserialization.
///
/// This wrapper ensures that timestamps are always serialized in RFC3339 format,
/// which is a subset of ISO-8601 suitable for internet protocols and APIs.
///
/// # Examples
///
/// ```
/// use serde_json::json;
/// 
/// let time = Timestamptz(time::OffsetDateTime::now_utc());
/// let json = serde_json::to_string(&time)?;
/// // Results in an RFC3339 string like: "2024-12-24T18:30:00Z"
/// ```
///
/// # Notes
/// 
/// While OffsetDateTime has built-in serialization, it produces an array of integers
/// which is not suitable for API responses. This wrapper provides a more appropriate
/// string format.
#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct Timestamptz(pub OffsetDateTime);

impl Serialize for Timestamptz {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Format the timestamp as an RFC3339 string
        self.0
            .format(&Rfc3339)
            .map_err(serde::ser::Error::custom)
            .and_then(|formatted| serializer.serialize_str(&formatted))
    }
}

impl<'de> Deserialize<'de> for Timestamptz {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// Custom visitor for deserializing RFC3339 timestamps
        struct TimestampVisitor;

        impl<'de> Visitor<'de> for TimestampVisitor {
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

impl PartialEq for Timestamptz {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Timestamptz {}

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_timestamp_serialization() -> serde_json::Result<()> {
        let dt = OffsetDateTime::now_utc();
        let ts = Timestamptz(dt);
        let serialized = serde_json::to_string(&ts)?;
        
        // Should be in quotes as it's a JSON string
        assert!(serialized.starts_with('"'));
        assert!(serialized.ends_with('"'));
        
        // Should be able to parse it back
        let deserialized: Timestamptz = serde_json::from_str(&serialized)?;
        assert_eq!(ts, deserialized);
        
        Ok(())
    }

    #[test]
    fn test_timestamp_deserialization_error() {
        let result: Result<Timestamptz, _> = serde_json::from_str("\"invalid-date\"");
        assert!(result.is_err());
    }
}