use std::path::PathBuf;

use clap_verbosity_flag::Verbosity;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RawArgs {
    /// The packages to store
    #[arg()]
    pub packages: Vec<String>,

    /// Set stow dir to DIR (default is current dir)
    #[arg(short, long)]
    pub dir: Option<PathBuf>,

    /// Set target to DIR (default is parent of stow dir)
    #[arg(short = 'T', long)]
    pub target: Option<PathBuf>,

    /// Store the package names that follow this option
    #[arg(short = 'S', long)]
    pub store: Option<String>,

    /// Unstore the package names that follow this option
    #[arg(short = 'D', long)]
    pub delete: Option<String>,

    /// Restore (like stow -D followed by stow -S)
    #[arg(short = 'R', long)]
    pub restore: Option<String>,

    /// Ignore files ending in this Perl regex
    #[arg(long)]
    pub ignore: Option<String>,

    /// Don't stow files beginning with this Perl regex
    /// if the file is already stowed to another package
    #[arg(long)]
    pub defer: Option<String>,

    /// Force stowing files beginning with this Perl regex
    /// if the file is already stowed to another package
    #[arg(long)]
    pub r#override: Option<String>,

    /// (Use with care!)  Import existing files into stow package
    /// from target.  Please read docs before using.
    #[arg(long)]
    pub adopt: bool,

    /// Use legacy algorithm for unstoring
    #[arg(short = 'p', long)]
    pub compat: bool,

    /// Do not actually make any filesystem changes
    #[arg(short = 'n', long, alias = "no")]
    pub simulate: bool,

    #[command(flatten)]
    pub verbose: Verbosity,
}
