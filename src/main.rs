use clap::{arg, value_parser, Command};

mod migration;

use migration::Migrator;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let command = Command::new("postgres-migrator")
        .subcommand_required(true)
        .subcommand(
            Command::new("run")
                .arg(arg!(--url <URL>).value_parser(value_parser!(String)))
                .arg(arg!(--path <PATH>).value_parser(value_parser!(String)))
                .arg(arg!(--username <USERNAME>).value_parser(value_parser!(String)))
                .arg(arg!(--description <DESCRIPTION>).value_parser(value_parser!(String))),
        );

    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            let url = sub_matches
                .get_one::<String>("url")
                .expect("missing '--url'");
            let path = sub_matches
                .get_one::<String>("path")
                .expect("missing '--path'");
            let username = sub_matches
                .get_one::<String>("username")
                .expect("missing '--username'");
            let description = sub_matches
                .get_one::<String>("description")
                .expect("missing '--description'");

            let migrator = Migrator::new(url, path, username, description).await?;
            migrator.migrate().await?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
