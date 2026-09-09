fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "ring")]
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("fresh consumer process must accept its chosen provider");

    // Constructors must work even for HTTPS destinations. No network requests
    // are made: this test does not need credentials or external API access.
    let endpoint = "https://localhost";
    let _clob = sdk::clob::Client::new(endpoint, sdk::clob::Config::default())?;
    let _data = sdk::data::Client::new(endpoint)?;
    let _gamma = sdk::gamma::Client::new(endpoint)?;
    let _bridge = sdk::bridge::Client::new(endpoint)?;
    println!("SDK TLS client construction passed");
    Ok(())
}
