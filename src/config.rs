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
/// Show secret key and public key for seed phrase. Show public key for secret key.
pub struct Config {
    #[clap(flatten)]
    pub input: Args,

    #[arg(long)]
    /// Vault to create the item in 1Password when using the create option
    ///
    /// When creating an item in 1Password, the vault name is required to define in which vault to create the item.
    pub vault: Option<String>,

    #[arg(long)]
    /// Account ID of the NEAR account associated with the key
    ///
    /// You can add an account id of a NEAR account to the key when adding it to 1Password. If you specify an account name, an additional field "Key Pair JSON" will be generated, which can be used to authenticale the NEAR CLI tool using 1Password.
    pub account: Option<String>,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct Args {
    #[arg(long, conflicts_with = "vault")]
    /// Seed phrase
    ///
    /// Pass a seed phrase to display the corresponding secret key and public key
    pub seed: Option<String>,

    #[arg(long, conflicts_with = "vault")]
    /// Secret key
    ///
    /// Pass a secret key to display the corresponding public key
    pub key: Option<String>,

    #[arg(long, requires = "vault")]
    /// Create an item in 1Password with the given name
    ///
    /// Create an item in 1Password with seed, secret key and public key. Do not output anything. The 1Password CLI tool must be installed. Specifying a vault to store the item in is required.
    pub create: Option<String>,
}
