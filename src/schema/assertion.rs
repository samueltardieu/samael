use super::{attribute_statement::AttributeStatement, AuthnStatement, Conditions, Issuer, Subject};
use crate::{
    attribute::{Attribute, NAME_FORMAT_URI},
    signature::Signature,
    utils::UtcDateTime,
};
use snafu::Snafu;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(
    namespace = "ds: http://www.w3.org/2000/09/xmldsig#",
    namespace = "saml: urn:oasis:names:tc:SAML:2.0:assertion",
    namespace = "xsd: http://www.w3.org/2001/XMLSchema"
)]
pub struct Assertion {
    #[yaserde(attribute, rename = "ID")]
    pub id: String,
    #[yaserde(attribute, rename = "IssueInstant")]
    pub issue_instant: UtcDateTime,
    #[yaserde(attribute, rename = "Version")]
    pub version: String,
    #[yaserde(rename = "Issuer", prefix = "saml")]
    pub issuer: Issuer,
    #[yaserde(rename = "Signature", prefix = "ds")]
    pub signature: Option<Signature>,
    #[yaserde(rename = "Subject", prefix = "saml")]
    pub subject: Option<Subject>,
    #[yaserde(rename = "Conditions", prefix = "saml")]
    pub conditions: Option<Conditions>,
    #[yaserde(rename = "AuthnStatement", prefix = "saml")]
    pub authn_statements: Vec<AuthnStatement>,
    #[yaserde(rename = "AttributeStatement", prefix = "saml")]
    pub attribute_statements: Vec<AttributeStatement>,
}

impl Assertion {
    pub fn attributes_by_name_and_format(&self, name: &str, name_format: &str) -> Vec<&Attribute> {
        self.attribute_statements
            .iter()
            .flat_map(|attribute_statement| {
                attribute_statement.attributes.iter().filter(|attr| {
                    attr.name_format.as_deref() == Some(name_format)
                        && attr.name.as_deref() == Some(name)
                })
            })
            .collect()
    }

    pub fn attributes_by_uri(&self, uri: &str) -> Vec<&Attribute> {
        self.attributes_by_name_and_format(uri, NAME_FORMAT_URI)
    }

    pub fn attribute_values(&self, uri: &str) -> Vec<&str> {
        self.attributes_by_uri(uri)
            .into_iter()
            .flat_map(|attr| {
                attr.values
                    .iter()
                    .filter(|v| {
                        v.attribute_type
                            .as_deref()
                            .map(|t| t == "XSString")
                            .unwrap_or(true)
                    })
                    .flat_map(|v| v.value.as_deref())
            })
            .collect()
    }

    pub fn attribute_value(&self, uri: &str) -> Result<&str, Error> {
        match &self.attribute_values(uri)[..] {
            &[v] => Ok(v),
            &[] => Err(Error::NotFound {
                uri: uri.to_owned(),
            }),
            v => Err(Error::NotUnique {
                uri: uri.to_owned(),
                count: v.len(),
            }),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Cannot find attribute {uri} by uri"))]
    NotFound { uri: String },
    #[snafu(display("Multiple ({count}) attributes found for uri {uri}"))]
    NotUnique { uri: String, count: usize },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attribute::*;

    #[test]
    fn attribute_value() {
        let statement = AttributeStatement {
            attributes: vec![
                Attribute {
                    name: Some("urn:dummy".to_owned()),
                    name_format: Some(NAME_FORMAT_URI.to_owned()),
                    values: vec![AttributeValue {
                        attribute_type: Some("XSInteger".to_owned()),
                        value: Some("23".to_owned()),
                    }],
                    ..Default::default()
                },
                Attribute {
                    name: Some(MAIL_URI.to_owned()),
                    name_format: Some(NAME_FORMAT_URI.to_owned()),
                    values: vec![AttributeValue {
                        attribute_type: Some("XSString".to_owned()),
                        value: Some("foo@example.com".to_owned()),
                    }],
                    ..Default::default()
                },
                Attribute {
                    name: Some(SURNAME_URI.to_owned()),
                    name_format: Some(NAME_FORMAT_URI.to_owned()),
                    values: vec![AttributeValue {
                        value: Some("doe".to_owned()),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                Attribute {
                    name: Some(GIVEN_NAME_URI.to_owned()),
                    name_format: Some(NAME_FORMAT_URI.to_owned()),
                    values: vec![AttributeValue {
                        value: Some("john".to_owned()),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                Attribute {
                    name: Some(GIVEN_NAME_URI.to_owned()),
                    name_format: Some(NAME_FORMAT_URI.to_owned()),
                    values: vec![AttributeValue {
                        value: Some("colin".to_owned()),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
        };
        let assertion = Assertion {
            attribute_statements: vec![statement],
            ..Default::default()
        };
        // With explicit xsi:type=XSString
        assert_eq!(
            assertion.attribute_value(MAIL_URI).unwrap(),
            "foo@example.com"
        );
        // Without explicit xsi:type
        assert_eq!(assertion.attribute_value(SURNAME_URI).unwrap(), "doe");
        // With explicit xsi:type=XSInteger
        assert!(matches!(
            assertion.attribute_value("urn:dummy"),
            Err(Error::NotFound { .. })
        ));
        // With multiple results
        assert!(matches!(
            assertion.attribute_value(GIVEN_NAME_URI),
            Err(Error::NotUnique { count: 2, .. })
        ));
        // With multiple results
        assert_eq!(
            assertion.attribute_values(GIVEN_NAME_URI),
            vec!["john", "colin"],
        );
    }
}
