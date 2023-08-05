use std::{time::Duration, io::stdout};
use clap::Parser;

use termimage::ops;
use base64::{Engine as _, engine::general_purpose};
use image::{ImageFormat, load_from_memory_with_format, GenericImageView};

use crate::palette::*;
mod palette;

#[derive(Parser)]
#[command(name = "MCStat")]
#[command(author = "caek <me@caek.dev>")]
#[command(version = "1.0")]
#[command(about = "Shows the Server List Ping of a Minecraft server", long_about = None)]
struct Args {
    #[arg()]
    host: String,

    #[arg(value_parser = clap::value_parser!(u16).range(1..), default_value_t = 25565)]
    port: u16,

    #[arg(long)]
    favicon: bool,

    #[arg(long)]
    player_sample: bool,

    #[arg(long, requires_if("true", "favicon"), default_value_t = 32)]
    size: u32
}

fn main() {
    let args = Args::parse();
    let host = format!("{}:{}", args.host, args.port);

    let result = mcping::get_status(mcping::Java {
        server_address: host,
        timeout: Some(Duration::from_secs(2)),
    });
    
    match result {
        Ok((latency, response)) => {
            let core_txt = color(String::from("Core"), Palette::OVERLAY2);
            println!("{:08}", core_txt);

            let ping_txt = color(String::from("Ping"), Palette::TEAL);
            let ping_val = ping_color(latency/2);
            println!("- {:08} {}", ping_txt, ping_val);

            let version_txt = color(String::from("Version"), Palette::TEAL);
            let version_val = color(response.version.name, Palette::GREEN);
            println!("- {:08} {}", version_txt, version_val);

            let players_txt = color(String::from("Players"), Palette::TEAL);
            let players_val_on = color(response.players.online.to_string(), Palette::GREEN);
            let players_val_max = color(response.players.max.to_string(), Palette::GREEN);
            println!("- {:08} {}/{}", players_txt, players_val_on, players_val_max);

            println!("");
            let motd_txt = color(String::from("MOTD"), Palette::OVERLAY1);
            let motd_val = mc_color(response.description.text().to_string());
            println!("{}\n{}\n", motd_txt, motd_val);

            if args.favicon {
                let motd_txt = color(String::from("Favicon"), Palette::OVERLAY1);
                if response.favicon.is_none() {
                    let no_motd_str = color(String::from("This server does not have a favicon."), Palette::TEXT);
                    println!("{}\n{}", motd_txt, no_motd_str);
                } else {
                    println!("{}", motd_txt);
                    let b64_image = &response.favicon.unwrap()[22..];
                    let bytes = general_purpose::STANDARD.decode(b64_image).expect("An error occured while decoding the favicon (base64)");
                    let image = load_from_memory_with_format(&bytes, ImageFormat::Png).expect("An error occured while decoding the image (png)");
                    let size = ops::image_resized_size(image.dimensions(), (args.size, args.size), true);
                    let resized = ops::resize_image(&image, size);

                    ops::write_ansi_truecolor(&mut stdout(), &resized);
                }
                println!("\n");
            }

            if args.player_sample {
                let sample_txt = color(String::from("Player Sample"), Palette::OVERLAY1);
                if response.players.sample.is_none() {
                    let no_sample_str = color(String::from("This server did not send a sample."), Palette::TEXT);
                    println!("{}\n{}", sample_txt, no_sample_str);
                } else {
                    let player_sample = response.players.sample.unwrap();
                    // Wow, that's long.
                    let player_sample_names = player_sample.iter().map(|player| &player.name).collect::<Vec<&String>>().iter().map(|s| s.to_string()).collect::<Vec<_>>();

                    let sample_val = color(player_sample_names.join(", "), Palette::TEXT);
                    println!("{}\n{}", sample_txt, sample_val);
                }
            }
        }

        Err(err) => {
            let error_msg = format!("{}", err);
            println!("{}", color(error_msg, Palette::RED));
        }
    }
}