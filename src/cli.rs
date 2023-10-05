use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
/// A voxel game inspired by Minecraft, CardLife, and a few other similar titles.
pub struct Cli {
    #[clap(long)]
    /// Bevy log level.
    pub bevy_log_level: Option<bevy::log::Level>,
}
