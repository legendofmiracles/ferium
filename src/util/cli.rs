//! Contains convenience wrappers for argument parsing using Clap
#![deny(missing_docs)] // All commands must have help/about statements

use std::path::PathBuf;

use clap::{AppSettings, Parser, Subcommand};

use super::json::ModLoaders;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(setting(AppSettings::SubcommandRequiredElseHelp))]
pub struct Ferium {
	#[clap(subcommand)]
	pub subcommand: SubCommands,
}

#[derive(Subcommand)]
pub enum SubCommands {
	#[clap(about("Add a Modrinth mod to the profile"))]
	AddModrinth {
		#[clap(help("The mod ID is specified as '</> PROJECT ID' in the right sidebar of the mod's Modrith page\nYou can also use the mod slug for this"))]
		mod_id: String,
	},
	#[clap(about("Add a GitHub repository to the profile"))]
	AddGithub {
		#[clap(help("The repository owner's username"))]
		owner: String,
		#[clap(help("The name of the repository"))]
		name: String,
	},
	#[clap(about("Add a CurseForge mod to the profile"))]
	AddCurseforge {
		#[clap(help("The project ID is specified as 'Project ID' in the 'About Project' sidebar of the mod's CurseForge page"))]
		project_id: i32,
	},
	#[clap(about("List all the mods in the profile with some their metadata"))]
	List {
		#[clap(short, long)]
		#[clap(help("Print more data about mods. Increases loading time"))]
		verbose: bool,
	},
	#[clap(setting(AppSettings::SubcommandRequiredElseHelp))]
	#[clap(about("Create, configure, or remove the current profile"))]
	Profile {
		#[clap(subcommand)]
		subcommand: ProfileSubCommands,
	},
	#[clap(about("Remove a mod or repository from the profile\nOptionally, provide a list of names of the mods to remove"))]
	Remove {
		#[clap(long)]
		#[clap(name("mod-name"))]
		#[clap(help("A case-insensitive name of a mod to remove\nYou can repeat this option to remove multiple mods\nIf one or more of the mod names provided does not exist, the program will error out without changing anything in the config"))]
		mod_names: Option<Vec<String>>,
	},
	#[clap(about("Switch between different profiles\nOptionally, provide the name of the profile to switch to"))]
	Switch {
		#[clap(long)]
		#[clap(help("The name of the profile to switch to"))]
		profile_name: Option<String>,
	},
	#[clap(about("Download and install the latest version of the mods specified"))]
	Upgrade {
		#[clap(long)]
		#[clap(help(
			"Do not show a picker when multiple compatible assets are found and pick the first one"
		))]
		no_picker: bool,
		#[clap(long)]
		#[clap(help(
			"Do not check for the full game version, only check for the major and minor versions"
		))]
		no_patch_check: bool,
	},
}

#[derive(Subcommand)]
pub enum ProfileSubCommands {
	#[clap(about(
		"Configure the current profile's Minecraft version, mod loader, and output directory\nOptionally, provide setting(s) to change as option(s)"
	))]
	Configure {
		#[clap(long)]
		#[clap(help("The Minecraft version to check compatibility for"))]
		game_version: Option<String>,
		#[clap(long)]
		#[clap(arg_enum)]
		#[clap(help("The mod loader to check compatibility for"))]
		mod_loader: Option<ModLoaders>,
		#[clap(long)]
		#[clap(help("The name of the profile"))]
		name: Option<String>,
		#[clap(long)]
		#[clap(help("The directory to output mods to"))]
		output_dir: Option<PathBuf>,
	},
	#[clap(about("Create a new profile\nOptionally, provide ALL the options to create the profile without the UI"))]
	Create {
		#[clap(long)]
		#[clap(help("The Minecraft version to check compatibility for"))]
		game_version: Option<String>,
		#[clap(long)]
		#[clap(help("Do not check whether the game version exists or not"))]
		force_game_version: bool,
		#[clap(long)]
		#[clap(arg_enum)]
		#[clap(help("The mod loader to check compatibility for"))]
		mod_loader: Option<ModLoaders>,
		#[clap(long)]
		#[clap(help("The name of the profile"))]
		name: Option<String>,
		#[clap(long)]
		#[clap(help("The directory to output mods to"))]
		output_dir: Option<PathBuf>,
	},
	#[clap(about("Delete a profile\nOptionally, provide the name of the profile to delete\nAfter deletion, the first profile will be selected"))]
	Delete {
		#[clap(long)]
		#[clap(help("The name of the profile to delete"))]
		profile_name: Option<String>,
	},
	#[clap(about("List all the profiles with their data"))]
	List,
}
