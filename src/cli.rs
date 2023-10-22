use clap::Parser;
use crate::asset_manager::AssetCheckBuildBehavior;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
/// A voxel game inspired by Minecraft, CardLife, and a few other similar titles
pub struct Cli {
    #[clap(long)]
    /// Bevy log level
    pub bevy_log_level: Option<bevy::log::Level>,

    #[clap(short, long)]
    /// Debug messages (Does not include Bevy debug messages)
    pub debug: bool,

    #[clap(long)]
    /// Quit before the actual game gets started up
    pub quit_before_game: bool,

    #[clap(long)]
    /// Should assets be (re)built or not?
    pub build_assets: Option<AssetCheckBuildBehavior>,

    #[clap(short, long)]
    /// Use the debug editor?
    pub editor: bool,
}
