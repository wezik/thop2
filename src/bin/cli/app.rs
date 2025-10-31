use ::clap::Parser;
use crab_hop::{
    app::bootstrap::{thop_service, RealThopService},
    domain::{
        error::DomainError,
        template,
        thop_service::{CreateCommand, DeleteCommand, OpenCommand, ThopServicePort},
    },
};

use crate::clap::{Clap, ClapCommands};

mod clap;

struct Cli {
    thop: RealThopService,
}

impl Cli {
    pub fn run(&self, args: Vec<String>) -> Result<(), DomainError> {
        let clap = Clap::parse_from(args);
        let command = clap.command.unwrap_or(ClapCommands::Open { path: None });

        return match command {
            ClapCommands::Open { path } => {
                let command = OpenCommand {
                    path: path.map(|path| template::Path(path)),
                };
                self.thop.open(command)
            }
            ClapCommands::Create { name, path } => {
                let command = CreateCommand {
                    name: name.map(|name| template::Name(name)),
                    path: path.map(|path| template::Path(path)),
                };
                self.thop.create(command)
            }
            ClapCommands::Delete { path } => {
                let command = DeleteCommand {
                    path: path.map(|path| template::Path(path)),
                };
                self.thop.delete(command)
            }
        };
    }
}

fn main() -> Result<(), DomainError> {
    let args = std::env::args().collect();
    let cli = Cli {
        thop: thop_service(),
    };
    cli.run(args)
}
