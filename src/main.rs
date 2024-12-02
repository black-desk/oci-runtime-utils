// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-CopyrightText: 2024, Chen Linxuan <me@black-desk.cn>

#![feature(trait_upcasting)]

mod cli;
mod cmds;
mod config;
mod inspect;
mod patch;
mod printer;

use rand::distributions::DistString;

fn main() -> std::io::Result<()> {
    use clap::Parser;
    let matches = cli::App::parse();

    colog::default_builder()
        .filter_level(match matches.verbose {
            0 => log::LevelFilter::Warn,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .init();

    if matches.markdown_help {
        clap_markdown::print_help_markdown::<cli::App>();
        return Ok(());
    }

    match matches.command.unwrap() {
        cli::Command::True {} => Ok(()),
        cli::Command::Inspect { format, command } => {
            let printer: &dyn printer::Printer = match format {
                cli::InspectFormat::Json => &printer::Json::new(),
                cli::InspectFormat::Yaml => &printer::YAML::new(),
                cli::InspectFormat::Toml => &printer::TOML::new(),
            };

            if command.is_none() {
                cmds::inspect::all(printer)?;
                return Ok(());
            }

            match command.unwrap() {
                cli::InspectCommands::Mountinfo {} => cmds::inspect::mountinfo(printer),
                cli::InspectCommands::Capabilities {} => cmds::inspect::capabilities(printer),
                cli::InspectCommands::Directory { path } => {
                    cmds::inspect::directory(printer, path.to_str().unwrap())
                }
                cli::InspectCommands::File { path } => {
                    cmds::inspect::file(printer, path.to_str().unwrap())
                }
            }
        }
        cli::Command::Run {
            runtime,
            container_id,
        } => cmds::run(
            &runtime,
            container_id
                .unwrap_or(
                    rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
                )
                .as_str(),
        ),
        cli::Command::Bundle { bundle } => cmds::bundle(&bundle),
        cli::Command::Spec { output } => cmds::spec(&output),
        cli::Command::Patch {
            input,
            output,
            patch,
            with_utils,
            commands,
        } => cmds::patch(&input, &output, &patch, with_utils, &commands),
    }
}
