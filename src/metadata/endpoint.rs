use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, YaDeserialize, YaSerialize)]
pub struct Endpoint {
    #[yaserde(attribute, rename = "Binding")]
    pub binding: String,
    #[yaserde(attribute, rename = "Location")]
    pub location: String,
    #[yaserde(attribute, rename = "ResponseLocation")]
    pub response_location: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, YaDeserialize, YaSerialize)]
pub struct IndexedEndpoint {
    #[yaserde(attribute, rename = "Binding")]
    pub binding: String,
    #[yaserde(attribute, rename = "Location")]
    pub location: String,
    #[yaserde(attribute, rename = "ResponseLocation")]
    pub response_location: Option<String>,
    #[yaserde(attribute)]
    pub index: u16,
    #[yaserde(attribute, rename = "isDefault")]
    pub is_default: Option<bool>,
}
