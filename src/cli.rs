use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
/// A voxel game inspired by Minecraft, CardLife, and a few other similar titles.
pub struct Cli {
    #[clap(long)]
    /// Bevy log level
    pub bevy_log_level: Option<bevy::log::Level>,

    #[clap(long)]
    /// Rebuild assets even if there is no need to
    pub rebuild_assets: bool,

    #[clap(long)]
    /// Debug messages (Does not include Bevy debug messages)
    pub debug: bool,

    #[clap(long)]
    /// Quit before the actual game gets started up.
    pub quit_before_game: bool,
}
