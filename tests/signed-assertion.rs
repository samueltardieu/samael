use samael::{
    metadata::{ContactPerson, ContactType, EntityDescriptor},
    service_provider::ServiceProviderBuilder,
    utils::UtcDateTime,
};

#[test]
fn test_response() {
    openssl_probe::init_ssl_cert_env_vars();

    let idp_metadata: EntityDescriptor =
        yaserde::de::from_str(include_str!("../test_vectors/samltest_id_metadata.xml")).unwrap();

    let cert = openssl::x509::X509::from_pem(include_bytes!("../examples/cert.cer")).unwrap();
    let private_key =
        openssl::rsa::Rsa::private_key_from_pem(include_bytes!("../examples/privatekey.pem"))
            .unwrap();

    let sp = ServiceProviderBuilder::default()
        .entity_id("fr.test.test".to_string())
        .key(private_key)
        .certificate(cert)
        .allow_idp_initiated(true)
        .contact_person(ContactPerson {
            surname: Some("Bob".to_string()),
            contact_type: ContactType::Technical.value().to_string(),
            ..ContactPerson::default()
        })
        .idp_metadata(idp_metadata)
        .acs_url("http://localhost:8080/saml/acs".to_string())
        .slo_url("http://localhost:8080/saml/slo".to_string())
        .build()
        .unwrap();

    unsafe { UtcDateTime::set_now("2022-02-08T15:53:10.421Z".parse().unwrap()) };

    sp.parse_response(
        include_str!("../test_vectors/signed_response.txt"),
        &["dummy"],
    )
    .unwrap();
}
