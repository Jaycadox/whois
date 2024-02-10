use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Registrar {
    iana_id: String,
    name: String,
    url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Contact {
    name: String,
    organization: String,
    street_address: String,
    city: String,
    region: String,
    zip_code: String,
    country: String,
    phone: String,
    fax: String,
    email: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DomainInfo {
    domain: String,
    domain_id: String,
    status: String,
    create_date: String,
    update_date: String,
    expire_date: String,
    domain_age: i32,
    whois_server: String,
    registrar: Registrar,
    registrant: Contact,
    admin: Contact,
    tech: Contact,
    billing: Contact,
    nameservers: Vec<String>,
}

fn get_api_key() -> String {
    let config = dirs::config_dir().expect("Unable to find config path");
    let conf_file = config.join(".whois");
    if !std::path::Path::new(&conf_file).exists() {
        eprintln!("Unable to find config file containing API key.");
        eprintln!("Trying to find config file at: {}", conf_file.display());
        std::process::exit(1);
    }

    std::fs::read_to_string(conf_file).expect("Unable to read config file")
}

fn create_api_url(api_key: &str, domain: &str) -> String {
    // TODO: use proper url parsing library
    format!("https://api.ip2whois.com/v2?key={api_key}&domain={domain}")
}

fn main() {
    let api_key = get_api_key();
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: whois [domain]");
        return;
    }

    let domain = &args[1];

    let url = create_api_url(&api_key, domain);
    let resp = ureq::get(&url)
        .call()
        .expect("Failed to make network request");

    let contents = resp.into_string().expect("Unable to parse response body");
    let info = serde_json::from_str::<DomainInfo>(&contents).expect("Unable to parse response");
    println!("{info:#?}");
}
