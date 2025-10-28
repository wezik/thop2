use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "thop", about = "Thop CLI")]
pub struct Clap {
    #[command(subcommand)]
    pub command: Option<ClapCommands>,
}

#[derive(Subcommand, Debug)]
pub enum ClapCommands {
    /// Opens a template
    Open {
        /// Template path
        #[arg(short = 'p', long = "path")]
        path: Option<String>,
    },

    /// Creates a new template
    Create {
        /// Template name
        #[arg(short = 'n', long = "name")]
        name: Option<String>,
        /// Template path
        #[arg(short = 'p', long = "path")]
        path: Option<String>,
    },

    /// Deletes a template
    Delete {
        /// Template path
        #[arg(short = 'p', long = "path")]
        path: Option<String>,
    },
}
