use std::fmt::{Display, Formatter};
use std::{fs, io};
use std::io::{BufRead};
use chrono::{Local};
use rcgen::{DnType, DnValue, BasicConstraints, IsCa, KeyUsagePurpose, ExtendedKeyUsagePurpose, DistinguishedName};
use time::{OffsetDateTime, Duration};
use rand::Rng;

struct SubjectFields {
    common_name: String,
    organization: String,
    organization_unit: String,
    country: String,
}


impl Display for SubjectFields {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "common_name: {}, organization: {}, organization_unit: {}, country: {}",
               self.common_name, self.organization, self.organization_unit, self.country)
    }
}

impl SubjectFields {
    fn new() -> SubjectFields {
        SubjectFields{
            common_name: "".to_string(),
            organization: "".to_string(),
            organization_unit: "".to_string(),
            country: "".to_string()
        }
    }
}


pub fn generate_certs(root_cn: &str, signing_cn: &str, expired: &bool) {
    let ts = Local::now().timestamp();

    let server_subject = get_user_input("server");
    let client_subject = get_user_input("client");

    fs::create_dir_all(format!("certs/{}", ts)).expect("TODO: panic message");
    fs::create_dir_all(format!("certs/{}", ts+1)).expect("");
    let root_cert = create_root_cert(root_cn);
    let signing_cert = create_root_cert(signing_cn);
    let server_cert = create_cert(&server_subject, *expired, "server");
    let client_cert = create_cert(&client_subject, *expired, "client");

    let serialized_root_cert = save_self_signed_cert(&root_cert, "certs/root-ca.crt", "certs/root-ca.key");
    let serialized_signing_cert = save_signed_cert(&signing_cert, &root_cert, "certs/signing-ca.crt", "certs/signing-ca.key");
    let serialized_server_cert = save_signed_cert(&server_cert, &signing_cert,
           format!("certs/{}/server.crt", ts).as_str(), format!("certs/{}/server.key", ts).as_str());
    let serialized_client_cert = save_signed_cert(&client_cert, &signing_cert,
          format!("certs/{}/client.crt", ts+1).as_str(), format!("certs/{}/client.key", ts + 1).as_str());

    println!("server bundle");
    save_cert_bundle(vec![serialized_server_cert.as_str(),
                          serialized_signing_cert.as_str(),
                          serialized_root_cert.as_str()],
                    format!("certs/{}/server-bundle.crt", ts).as_str());

    println!("client bundle");
    save_cert_bundle(vec![serialized_client_cert.as_str(),
                          serialized_signing_cert.as_str(),
                          serialized_root_cert.as_str()],
                    format!("certs/{}/client-bundle.crt", ts + 1).as_str());

    println!("server cert path: {}", ts);
    println!("client cert path: {}", ts + 1);
}

fn get_user_input(s: &str) -> SubjectFields {
    let mut subject = SubjectFields::new();

    println!("Enter {} common name: ", s);
    subject.common_name = io::stdin().lock().lines().next().expect("next").expect("lines");

    println!("Enter {} organization: ", s);
    subject.organization = io::stdin().lock().lines().next().expect("next").expect("lines");

    println!("Enter {} organization unit: ", s);
    subject.organization_unit = io::stdin().lock().lines().next().expect("next").expect("lines");

    println!("Enter {} country: ", s);
    subject.country = io::stdin().lock().lines().next().expect("next").expect("lines");

    subject
}

fn create_root_cert(cn: &str) -> rcgen::Certificate {
    let mut params = rcgen::CertificateParams::new(vec!["".to_string()]);

    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, DnValue::PrintableString(cn.to_string()));

    params.distinguished_name = dn;
    params.not_before = OffsetDateTime::now_utc();
    params.not_after = OffsetDateTime::now_utc().checked_add(Duration::days(365)).expect("");
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign, KeyUsagePurpose::KeyAgreement];
    params.extended_key_usages = vec![ExtendedKeyUsagePurpose::CodeSigning, ExtendedKeyUsagePurpose::ClientAuth, ExtendedKeyUsagePurpose::ServerAuth];
    params.serial_number = Some(rand::thread_rng().gen::<u64>());

    rcgen::Certificate::from_params(params).expect("")
}

fn create_cert(subject: &SubjectFields, expired: bool, auth_type: &str) -> rcgen::Certificate {
    let mut params = rcgen::CertificateParams::new(vec!["".to_string()]);

    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, DnValue::PrintableString(subject.common_name.to_string()));
    dn.push(DnType::OrganizationName, DnValue::PrintableString(subject.organization.to_string()));
    dn.push(DnType::OrganizationalUnitName, DnValue::PrintableString(subject.organization_unit.to_string()));
    dn.push(DnType::CountryName, DnValue::PrintableString(subject.country.to_string()));

    params.distinguished_name = dn;
    if expired {
        params.not_before = OffsetDateTime::now_utc().checked_sub(Duration::days(30)).expect("");
        params.not_after = OffsetDateTime::now_utc().checked_add(Duration::days(1)).expect("");
    } else {
        params.not_before = OffsetDateTime::now_utc();
        params.not_after = OffsetDateTime::now_utc().checked_add(Duration::days(365)).expect("");
    }

    params.is_ca = IsCa::ExplicitNoCa;
    params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
    match auth_type {
        "server" => params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth],
        "client" => params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ClientAuth],
        _ => {params.extended_key_usages = vec![ExtendedKeyUsagePurpose::Any]},
    }
    params.serial_number = Some(rand::thread_rng().gen::<u64>());

    rcgen::Certificate::from_params(params).expect("")
}

fn save_self_signed_cert(cert: &rcgen::Certificate, path: &str, key_path: &str) -> String {
    let mut serialized_cert = String::new();
    match cert.serialize_pem() {
        Ok(sc) => {
            fs::write(path, &sc).expect("TODO: panic message");
            serialized_cert = sc;
        }
        Err(e) => {
            println!("error writing file: {}", e);
        }
    }

    fs::write(key_path, cert.serialize_private_key_pem()).expect("");

    serialized_cert
}

fn save_signed_cert(cert: &rcgen::Certificate, signing_cert: &rcgen::Certificate, path: &str, key_path: &str) -> String {
    let mut serialized_cert = String::new();
    match cert.serialize_pem_with_signer(&signing_cert) {
        Ok(sc) => {
            fs::write(path, &sc).expect("");
            serialized_cert = sc;
        }
        Err(e) => println!("{}", e)
    }

    fs::write(key_path, cert.serialize_private_key_pem()).expect("");

    serialized_cert
}

fn save_cert_bundle(chain: Vec<&str>, path: &str) {
    let mut cert_bundle = String::new();
    for i in chain {
        cert_bundle.push_str(i)
    }

    fs::write(path, cert_bundle).expect("");
}
