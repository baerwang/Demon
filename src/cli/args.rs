use std::convert::Infallible;
use std::path::PathBuf;

use clap::ArgAction::Set;
use clap::{Args, Parser, Subcommand};
use headless_chrome::browser::default_executable;

#[derive(Debug, Parser)]
#[command(author, version, about, subcommand_precedence_over_arg = true)]
pub struct CLi {
    /// Target to Website,Support Multi '--target https://example.com https://testphp.vulnweb.com'
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub target: Vec<String>,
    /// Custom Http Headers,support multi '--custom-headers Server:example Cookie:baerwang'
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub custom_headers: Vec<String>,
    /// Robots Exclusion Protocol
    #[arg(short, long, default_value_t = false, action = Set)]
    pub robots: bool,
    /// Authenticate username
    #[arg(short, long)]
    pub username: Option<String>,
    /// Authenticate password
    #[arg(short, long)]
    pub password: Option<String>,
    #[command(subcommand)]
    #[clap(value_parser = opt_default)]
    pub opt: Option<Opt>,
}

#[derive(Debug, Subcommand)]
pub enum Opt {
    /// chromium setup
    Chromium(Chromium),
}

#[derive(Debug, Args)]
pub struct Chromium {
    /// Path for Chrome or Chromium.
    pub path: Option<PathBuf>,
    /// Determines whether to run headless version of the browser. Defaults to true.
    #[arg(long, default_value_t = true, action = Set)]
    pub headless: bool,
    /// Determines whether to run the browser with a sandbox.
    #[arg(long, default_value_t = true, action = Set)]
    pub sandbox: bool,
    /// Determines whether SSL certificates should be verified.
    #[arg(long, default_value_t = true, action = Set)]
    pub ignore_certificate_errors: bool,
    /// User Data (Profile) to use If unspecified, a new temp directory is created and used on every launch.
    pub user_data_dir: Option<PathBuf>,
    /// Setup the proxy server for headless chrome instance.
    pub proxy: Option<String>,
}

#[allow(dead_code)]
fn opt_default(o: Option<Opt>) -> Result<Option<Opt>, Infallible> {
    if o.is_none() {
        return Ok(Some(Opt::Chromium(Chromium {
            path: Some(default_executable().unwrap()),
            headless: true,
            sandbox: true,
            ignore_certificate_errors: true,
            user_data_dir: None,
            proxy: None,
        })));
    }
    Ok(o)
}
