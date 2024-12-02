// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-CopyrightText: 2024, Chen Linxuan <me@black-desk.cn>

#[derive(clap::Parser, Debug)]
#[command(
    version,
    about = "Utilities for working on OCI runtime CLI",
    long_about = include_str!("../docs/long_about.txt").trim(),
    after_help = include_str!("../docs/after_help.txt").trim(),
)]
pub struct App {
    #[arg(short, action = clap::ArgAction::Count, help = "Set verbosity level")]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(long, hide = true)]
    pub markdown_help: bool,
}

#[derive(clap::Subcommand, Debug)]
pub enum InspectCommands {
    #[command(about = "Inspect contents of a directory")]
    Directory {
        #[arg(help = "Directory to list")]
        path: std::path::PathBuf,
    },

    #[command(about = "Inspect a text file")]
    File {
        #[arg(help = "File to cat")]
        path: std::path::PathBuf,
    },

    #[command(about = "Inspect /proc/self/mountinfo")]
    Mountinfo {},

    #[command(about = "Inspect capabilities")]
    Capabilities {},
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum InspectFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    #[command(about = "Return 0")]
    True,

    #[command(
        about = "Inspect the system environment of current namespace",
        long_about = include_str!("../docs/inspect_long_about.txt").trim()
    )]
    Inspect {
        #[arg(short, long, default_value = "yaml", help = "Output format")]
        format: InspectFormat,

        #[command(subcommand)]
        command: Option<InspectCommands>,
    },

    #[command(about = "Run a container with specified OCI runtime")]
    Run {
        #[arg(help = "Absolute file path to OCI runtime CLI program")]
        runtime: std::path::PathBuf,

        #[arg(help = "Optional container id. A random id will be generated if not provided")]
        container_id: Option<String>,
    },

    #[command(
        about = "Create a minimal OCI bundle",
        long_about = include_str!("../docs/bundle_long_about.txt").trim()
    )]
    Bundle {
        #[arg(
            short,
            long,
            default_value = ".",
            help = "Path to the generated bundle"
        )]
        bundle: std::path::PathBuf,
    },

    #[command(
        about = "Generated a minimal OCI runtime config file",
        long_about = include_str!("../docs/spec_long_about.txt").trim(),
    )]
    Spec {
        #[arg(short, long, default_value = "config.json", help = "Generated file")]
        output: std::path::PathBuf,
    },

    #[command(
        about = "Apply a json-patch to an OCI runtime config file, then print result to stdout",
        long_about = include_str!("../docs/patch_long_about.txt").trim()
    )]
    Patch {
        #[arg(short, long, default_value = "config.json", help = "Input file")]
        input: std::path::PathBuf,
        #[arg(short, long, default_value = "/dev/stdout", help = "Output file")]
        output: std::path::PathBuf,
        #[arg(
            long,
            default_value_t = false,
            help = "Patch config.json to enable oci-runtime-utils in container"
        )]
        with_utils: bool,
        #[arg(
            default_values_t = ["/usr/local/bin/oci-runtime-utils".to_string(), "true".to_string()],
            help = "Patch config.json to run this command in container"
        )]
        commands: Vec<String>,
        #[arg(long, help = "The json-patch file to apply")]
        patch: Option<String>,
    },
}
