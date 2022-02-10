use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance")]
pub struct AttributeValue {
    #[yaserde(attribute, rename = "type", prefix = "xsi")]
    pub attribute_type: Option<String>,
    #[yaserde(text)]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(namespace = "saml: urn:oasis:names:tc:SAML:2.0:assertion")]
pub struct Attribute {
    #[yaserde(attribute, rename = "FriendlyName")]
    pub friendly_name: Option<String>,
    #[yaserde(attribute, rename = "Name")]
    pub name: Option<String>,
    #[yaserde(attribute, rename = "NameFormat")]
    pub name_format: Option<String>,
    #[yaserde(rename = "AttributeValue", prefix = "saml", default)]
    pub values: Vec<AttributeValue>,
}

pub static NAME_FORMAT_URI: &str = "urn:oasis:names:tc:SAML:2.0:attrname-format:uri";
pub static SUBJECT_ID_URI: &str = "urn:oasis:names:tc:SAML:attribute:subject-id";
pub static UID_URI: &str = "urn:oid:0.9.2342.19200300.100.1.1";
pub static TELEPHONE_NUMBER_URI: &str = "urn:oid:2.5.4.20";
pub static MAIL_URI: &str = "urn:oid:0.9.2342.19200300.100.1.3";
pub static SURNAME_URI: &str = "urn:oid:2.5.4.4";
pub static DISPLAY_NAME_URI: &str = "urn:oid:2.16.840.1.113730.3.1.241";
pub static GIVEN_NAME_URI: &str = "urn:oid:2.5.4.42";
pub static EDU_PERSON_ENTITLEMENT_URI: &str = "urn:oid:1.3.6.1.4.1.5923.1.1.1.7";
