use crate::crypto;
use crate::schema::AuthnRequest;
#[cfg(feature = "xmlsec")]
use crypto::verify_signed_xml;

use super::error::Error;

pub struct UnverifiedAuthnRequest<'a> {
    pub request: AuthnRequest,
    #[cfg_attr(not(feature = "xmlsec"), allow(unused))]
    xml: &'a str,
}

impl<'a> UnverifiedAuthnRequest<'a> {
    pub fn from_xml(xml: &'a str) -> Result<Self, String> {
        Ok(UnverifiedAuthnRequest {
            request: yaserde::de::from_str(xml)?,
            xml,
        })
    }

    pub fn get_certs_der(&self) -> Result<Vec<Vec<u8>>, Error> {
        let x509_certs = self
            .request
            .signature
            .as_ref()
            .ok_or(Error::NoSignature)?
            .key_info
            .first()
            .ok_or(Error::NoKeyInfo)?
            .x509_data
            .iter()
            .flat_map(|d| d.certificates.iter())
            .map(|cert| crypto::decode_x509_cert(cert.as_str()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| Error::InvalidCertificateEncoding)?;

        if x509_certs.is_empty() {
            return Err(Error::NoCertificate);
        }

        Ok(x509_certs)
    }

    #[cfg(feature = "xmlsec")]
    pub fn try_verify_self_signed(self) -> Result<VerifiedAuthnRequest, Error> {
        let xml = self.xml.as_bytes();
        self.get_certs_der()?
            .into_iter()
            .map(|der_cert| Ok(verify_signed_xml(xml, &der_cert, Some("ID"))?))
            .reduce(|a, b| a.or(b))
            .unwrap()
            .map(|()| VerifiedAuthnRequest(self.request))
    }

    #[cfg(feature = "xmlsec")]
    pub fn try_verify_with_cert(self, der_cert: &[u8]) -> Result<VerifiedAuthnRequest, Error> {
        verify_signed_xml(self.xml.as_bytes(), der_cert, Some("ID"))?;
        Ok(VerifiedAuthnRequest(self.request))
    }
}

pub struct VerifiedAuthnRequest(AuthnRequest);

impl std::ops::Deref for VerifiedAuthnRequest {
    type Target = AuthnRequest;
    fn deref(&self) -> &AuthnRequest {
        &self.0
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[cfg(feature = "xmlsec")]
    fn test_request_deserialize_and_serialize() {
        let authn_request_xml = include_str!("../../test_vectors/authn_request.xml");
        let unverified =
            super::UnverifiedAuthnRequest::from_xml(authn_request_xml).expect("failed to parse");
        let expected_verified = unverified
            .try_verify_self_signed()
            .expect("failed to verify self signed signature");
        let verified_request_xml = expected_verified
            .as_xml()
            .expect("Failed to serialize verified authn request");
        let reparsed_unverified = super::UnverifiedAuthnRequest::from_xml(&verified_request_xml)
            .expect("failed to parse");
        assert_eq!(reparsed_unverified.request, expected_verified.0);
    }
}
