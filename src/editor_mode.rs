use bevy::prelude::*;
use bevy_editor_pls::*;
use clap::Parser;

use crate::cli;

pub struct EditorModePlugin;

impl Plugin for EditorModePlugin {
    fn build(&self, app: &mut App) {
        let args = cli::Cli::parse();

        if args.editor {
            app.add_plugins(EditorPlugin::default());
        }
    }
}
