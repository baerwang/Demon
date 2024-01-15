use std::path::PathBuf;

use clap::ArgAction::Set;
use clap::{Args, Parser, Subcommand};
use headless_chrome::browser::default_executable;

#[derive(Debug, Parser)]
#[command(author, version, about, subcommand_precedence_over_arg = true)]
pub struct CLi {
    /// Number of concurrent transactions
    #[arg(short, long, default_value_t = 20, action = Set)]
    pub thread: usize,
    /// Target to Website,Support Multi-value '--target https://example.com http://testphp.vulnweb.com'
    #[arg(long, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub target: Vec<String>,
    /// Custom Http Headers,Support Multi-value '--custom-headers Server:example Cookie:baerwang'
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
    /// DEBUG ERROR WARN
    #[arg(short, long, default_value = "INFO", action = Set)]
    pub log_level: String,
    #[command(subcommand)]
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
    #[arg(long = "path")]
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
    #[arg(short, long)]
    pub user_data_dir: Option<PathBuf>,
    /// Setup the proxy server for headless chrome instance.
    #[arg(long = "proxy")]
    pub proxy: Option<String>,
}

impl Default for Opt {
    fn default() -> Self {
        Self::new()
    }
}

impl Opt {
    pub fn new() -> Self {
        Opt::Chromium(Chromium::new())
    }
}

impl Default for Chromium {
    fn default() -> Self {
        Self::new()
    }
}

impl Chromium {
    pub fn new() -> Self {
        Chromium {
            path: Some(default_executable().unwrap()),
            headless: true,
            sandbox: true,
            ignore_certificate_errors: true,
            user_data_dir: None,
            proxy: None,
        }
    }
}
