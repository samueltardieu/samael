use samael::metadata::{ContactPerson, ContactType, EntityDescriptor};
use samael::service_provider::ServiceProviderBuilder;
use std::collections::HashMap;
use warp::hyper::Uri;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    openssl_probe::init_ssl_cert_env_vars();

    let resp = reqwest::get("https://samltest.id/saml/idp")
        .await?
        .text()
        .await?;
    let idp_metadata: EntityDescriptor = yaserde::de::from_str(&resp)?;

    let cert = openssl::x509::X509::from_pem(include_bytes!("cert.cer"))?;
    let private_key = openssl::rsa::Rsa::private_key_from_pem(include_bytes!("privatekey.pem"))?;

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
        .idp_metadata(idp_metadata.clone())
        .acs_url("http://localhost:8080/saml/acs".to_string())
        .slo_url("http://localhost:8080/saml/slo".to_string())
        .build()?;

    let metadata = yaserde::ser::to_string(&sp.metadata()?)?;

    let metadata_route = warp::get()
        .and(warp::path("metadata"))
        .map(move || metadata.clone());

    let start_request_route = warp::get().map({
        let sp = sp.clone();
        move || {
            let endpoint = idp_metadata
                .idp_sso_descriptors
                .iter()
                .find_map(|desc| {
                    desc.single_sign_on_services
                        .iter()
                        .find(|sso| {
                            sso.binding == "urn:oasis:names:tc:SAML:2.0:bindings:HTTP-Redirect"
                        })
                        .map(|sso| sso.location.clone())
                })
                .unwrap();
            let authn_request = sp.make_authentication_request(&endpoint).unwrap();
            println!(
                "request = {}",
                yaserde::ser::to_string(&authn_request).unwrap()
            );
            let uri = authn_request
                .redirect(Some("http://localhost:8080/do-something-useful"))
                .unwrap();
            let uri = uri.to_string().parse::<Uri>().unwrap();
            println!("uri is {uri}");
            warp::redirect::found(uri)
        }
    });

    let acs_route = warp::post()
        .and(warp::path("acs"))
        .and(warp::body::form())
        .map(move |s: HashMap<String, String>| {
            let response = if let Some(encoded_resp) = s.get("SAMLResponse") {
                let t = sp
                    .parse_response(encoded_resp, &["testreq".to_string()])
                    .unwrap();
                let config = yaserde::ser::Config {
                    perform_indent: true,
                    ..Default::default()
                };
                yaserde::ser::to_string_with_config(&t, &config).unwrap()
            } else {
                String::from("<no response>")
            };
            let relay_state = s
                .get("RelayState")
                .cloned()
                .unwrap_or_else(|| String::from("<no relay state>"));
            format!("Relay state: {relay_state}\nResponse:\n{response}")
        });

    let saml_routes = warp::path("saml")
        .and(acs_route.or(metadata_route))
        .or(start_request_route);
    warp::serve(saml_routes).run(([127, 0, 0, 1], 8080)).await;
    Ok(())
}
