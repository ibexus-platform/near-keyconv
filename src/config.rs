#[derive(clap::Parser, Debug)]
#[command(name = "NEAR Key Converter")]
#[command(bin_name = "near-keyconv")]
#[command(author = "developer@ibexus.io")]
#[command(version = option_env!("CARGO_PKG_VERSION"))]
#[command(
    help_template = "{about-section}\nSupport: {author}\nVersion: {version}\n\n{usage-heading}\n\n{usage}\n\n{all-args}{tab}"
)]
/// NEAR Key Converter
///
/// Show signing/private key and verifying/public key for seed phrase. Show verifying/public key for signing/private key.
pub struct Config {
  #[clap(flatten)]
  pub input: Args,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct Args {
    #[arg(long, conflicts_with = "key")]
    /// Seed phrase
    ///
    /// Pass a seed phrase to display the corresponding signing/private key and verifying/public key
    pub seed: Option<String>,

    #[arg(long)]
    /// Signing/private key
    ///
    /// Pass a signing/private key to display the corresponding verifying/public key
    pub key: Option<String>,
}
