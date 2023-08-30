use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "connectingbackground.png")]
    pub background: Handle<Image>,
    #[asset(path = "pig.png")]
    pub pig: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "Roboto-Regular.ttf")]
    pub default_font: Handle<Font>,
}
