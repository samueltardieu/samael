use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(namespace = "md: urn:oasis:names:tc:SAML:2.0:metadata")]
pub struct EncryptionMethod {
    #[yaserde(attribute, rename = "Algorithm")]
    pub algorithm: String,
}
