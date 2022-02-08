use chrono::{DateTime, Duration, SecondsFormat, Utc};
use std::io::{Read, Write};
use std::ops::Add;
use yaserde::xml;
use yaserde::{YaDeserialize, YaSerialize};

static mut NOW: Option<DateTime<Utc>> = None;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct UtcDateTime(pub DateTime<Utc>);

impl UtcDateTime {
    pub fn now() -> Self {
        match unsafe { NOW } {
            Some(now) => Self(now),
            None => Self(Utc::now()),
        }
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn set_now(now: DateTime<Utc>) {
        NOW = Some(now);
    }
}

impl Default for UtcDateTime {
    fn default() -> Self {
        Self::now()
    }
}

impl Add<Duration> for &UtcDateTime {
    type Output = UtcDateTime;

    fn add(self, other: Duration) -> Self::Output {
        UtcDateTime(self.0 + other)
    }
}

impl YaDeserialize for UtcDateTime {
    fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        match (
            reader.next_event()?,
            reader.next_event()?,
            reader.next_event()?,
        ) {
            (
                xml::reader::XmlEvent::StartElement { .. },
                xml::reader::XmlEvent::Characters(s),
                xml::reader::XmlEvent::EndElement { .. },
            ) => Ok(UtcDateTime(
                s.parse().map_err(|e: chrono::ParseError| e.to_string())?,
            )),
            _ => Err("Malformed RFC3339 time attribute".to_string()),
        }
    }
}

impl YaSerialize for UtcDateTime {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
        writer
            .write(xml::writer::XmlEvent::Characters(
                &self.0.to_rfc3339_opts(SecondsFormat::Millis, true),
            ))
            .map_err(|e| e.to_string())
    }

    fn serialize_attributes(
        &self,
        attributes: Vec<xml::attribute::OwnedAttribute>,
        namespace: xml::namespace::Namespace,
    ) -> Result<
        (
            Vec<xml::attribute::OwnedAttribute>,
            xml::namespace::Namespace,
        ),
        String,
    > {
        Ok((attributes, namespace))
    }
}

pub fn gen_saml_response_id() -> String {
    format!("id{}", uuid::Uuid::new_v4())
}

pub fn gen_saml_assertion_id() -> String {
    format!("_{}", uuid::Uuid::new_v4())
}
