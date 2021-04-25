use std::env;

use anyhow::Result;
use obws::{
    common::FontFlags,
    requests::{
        custom::{Font, TextFt2SourceV2, SOURCE_TEST_FT2_SOURCE_V2},
        SourceSettings,
    },
    Client,
};
use rgb::RGBA8;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env::set_var("RUST_LOG", "obws=trace");
    pretty_env_logger::init();

    let client = Client::connect("localhost", 4444).await?;

    let version = client.general().get_version().await?;
    println!("{:#?}", version);

    client.login(env::var("OBS_PASSWORD").ok()).await?;

    let settings = client
        .sources()
        .set_source_settings::<serde_json::Value>(SourceSettings {
            source_name: "TEST-1",
            source_type: Some(SOURCE_TEST_FT2_SOURCE_V2),
            source_settings: &serde_json::to_value(&TextFt2SourceV2 {
                color1: RGBA8::new(255, 0, 0, 255),
                color2: RGBA8::new(0, 0, 255, 255),
                text: "Hello world!",
                font: Font {
                    flags: FontFlags::BOLD,
                    style: "Bold",
                    ..Font::default()
                },
                ..TextFt2SourceV2::default()
            })?,
        })
        .await?;

    println!("{:#?}", settings.source_settings);

    Ok(())
}
