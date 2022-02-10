use crate::attribute::Attribute;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(namespace = "saml: urn:oasis:names:tc:SAML:2.0:assertion")]
pub struct AttributeStatement {
    #[yaserde(rename = "Attribute", prefix = "saml")]
    pub(crate) attributes: Vec<Attribute>,
}
