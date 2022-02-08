use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, YaDeserialize, YaSerialize)]
pub struct Issuer {
    #[yaserde(attribute, rename = "NameQualifier")]
    pub name_qualifier: Option<String>,
    #[yaserde(attribute, rename = "SPNameQualifier")]
    pub sp_name_qualifier: Option<String>,
    #[yaserde(attribute, rename = "Format")]
    pub format: Option<String>,
    #[yaserde(attribute, rename = "SPProvidedID")]
    pub sp_provided_id: Option<String>,
    #[yaserde(text)]
    pub value: Option<String>,
}
