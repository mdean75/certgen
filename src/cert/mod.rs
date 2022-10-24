use std::fmt::{Display, Formatter};
use std::{fs, io};
use std::io::{BufRead, Write};
use std::ops::{Add, Sub};
use rcgen::{BasicConstraints, Certificate, DnType, DnValue, ExtendedKeyUsagePurpose, IsCa, KeyUsagePurpose};
use rcgen::DnValue::PrintableString;
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
    let server_subject = get_user_input("server");
    let client_subject = get_user_input("client");

    let root_cert = create_root_cert(root_cn);
    let signing_cert = create_root_cert(signing_cn);
    let server_cert = create_cert(&server_subject, *expired, "server");
    let client_cert = create_cert(&client_subject, *expired, "client");

    save_self_signed_cert(&root_cert, "root-ca.crt", "root-ca.key");
    save_signed_cert(&signing_cert, &root_cert, "signing-ca.crt", "signing-ca.key");
    save_signed_cert(&server_cert, &signing_cert, "server.crt", "server.key");
    save_signed_cert(&client_cert, &server_cert, "client.crt", "client.key");
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

    let mut dn = rcgen::DistinguishedName::new();
    dn.push(rcgen::DnType::CommonName, PrintableString(cn.to_string()));

    params.distinguished_name = dn;
    params.not_before = OffsetDateTime::now_utc();
    params.not_after = OffsetDateTime::now_utc().checked_add(Duration::days(365)).expect("");
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.key_usages = vec![KeyUsagePurpose::KeyCertSign, KeyUsagePurpose::CrlSign, KeyUsagePurpose::KeyAgreement];
    params.extended_key_usages = vec![ExtendedKeyUsagePurpose::CodeSigning];
    params.serial_number = Some(rand::thread_rng().gen::<u64>());

    rcgen::Certificate::from_params(params).expect("")
}

fn create_cert(subject: &SubjectFields, expired: bool, auth_type: &str) -> rcgen::Certificate {
    let mut params = rcgen::CertificateParams::new(vec!["".to_string()]);

    let mut dn = rcgen::DistinguishedName::new();
    dn.push(rcgen::DnType::CommonName, PrintableString(subject.common_name.to_string()));
    dn.push(DnType::OrganizationName, PrintableString(subject.organization.to_string()));
    dn.push(DnType::OrganizationalUnitName, PrintableString(subject.organization_unit.to_string()));
    dn.push(DnType::CountryName, PrintableString(subject.country.to_string()));

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

fn save_self_signed_cert(cert: &Certificate, path: &str, x: &str) {
    match cert.serialize_pem() {
        Ok(sc) => {
            fs::write(path, sc).expect("TODO: panic message");
        }
        Err(e) => {
            println!("error writing file");
        }
    }

    fs::write(path, cert.serialize_private_key_pem()).expect("");
}

fn save_signed_cert(cert: &Certificate, signing_cert: &Certificate, path: &str, x: &str) {
    match cert.serialize_pem_with_signer(&signing_cert) {
        Ok(sc) => {
            fs::write(path, sc).expect("")
        }
        Err(e) => println!("{}", e)
    }

    fs::write(path, cert.serialize_private_key_pem()).expect("");
}
