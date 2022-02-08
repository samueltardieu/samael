use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, YaDeserialize, YaSerialize)]
pub struct NameIdPolicy {
    #[yaserde(attribute, rename = "Format")]
    pub format: Option<String>,
    #[yaserde(attribute, rename = "SPNameQualifier")]
    pub sp_name_qualifier: Option<String>,
    #[yaserde(attribute, rename = "AllowCreate")]
    pub allow_create: Option<bool>,
}
