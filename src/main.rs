/*
 * Copyright 2021 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use anyhow::Result;
use clap::{crate_authors, crate_description, crate_version, App, Arg, ArgMatches};
use evdev::{Device, EventType, InputEventKind, Key};
use rodio::{Decoder, OutputStream, Sink};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Deserialize)]
struct Config {
    sounds_path: PathBuf,
    inputs_to_filenames: HashMap<String, String>,
}

fn parse_args() -> ArgMatches<'static> {
    App::new("Scanner Soundboard")
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Specify configuration file (e.g. `config.toml`)")
                .required(true)
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("input_device")
                .short("i")
                .long("input-device")
                .help("Specify input device (e.g. `/dev/input/event23`)")
                .required(true)
                .takes_value(true)
                .value_name("DEVICE"),
        )
        .get_matches()
}

fn load_config(path: &Path) -> Result<Config> {
    let text = read_to_string(path)?;
    let config: Config = toml::from_str(&text)?;
    Ok(config)
}

fn get_char(key: Key) -> Option<char> {
    match key {
        Key::KEY_1 => Some('1'),
        Key::KEY_2 => Some('2'),
        Key::KEY_3 => Some('3'),
        Key::KEY_4 => Some('4'),
        Key::KEY_5 => Some('5'),
        Key::KEY_6 => Some('6'),
        Key::KEY_7 => Some('7'),
        Key::KEY_8 => Some('8'),
        Key::KEY_9 => Some('9'),
        Key::KEY_0 => Some('0'),
        _ => None,
    }
}

fn play_sound(
    inputs_to_filenames: &HashMap<String, String>,
    input: &str,
    dir: &Path,
    sink: &Sink,
) -> Result<()> {
    match inputs_to_filenames.get(input.trim()) {
        Some(filename) => {
            let path = dir.join(filename);
            if !&path.exists() {
                eprintln!("Sound file {} does not exist.", path.display());
                return Ok(());
            }
            let source = load_source(&path)?;
            sink.append(source);
        }
        _ => (),
    }
    Ok(())
}

fn load_source(path: &Path) -> Result<Decoder<BufReader<File>>> {
    let file = BufReader::new(File::open(path)?);
    Ok(Decoder::new(file)?)
}

fn main() -> Result<()> {
    let args = parse_args();

    let config_filename = args.value_of("config").map(Path::new).unwrap();
    let config = load_config(config_filename)?;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.sleep_until_end();

    let input_device_path = args.value_of("input_device").unwrap();
    let mut input_device = Device::open(input_device_path)?;
    println!(
        "Opened input device \"{}\".",
        input_device.name().unwrap_or("unnamed device")
    );

    match input_device.grab() {
        Ok(_) => println!("Successfully obtained exclusive access to input device."),
        Err(error) => {
            eprintln!("Could not get exclusive access to input device: {}", error);
            exit(1);
        }
    }

    let mut read_chars = String::new();
    loop {
        for event in input_device.fetch_events()? {
            // Only handle pressed key events.
            if event.event_type() != EventType::KEY || event.value() == 1 {
                continue;
            }

            match event.kind() {
                InputEventKind::Key(Key::KEY_ENTER) => {
                    let input = read_chars.as_str();
                    play_sound(
                        &config.inputs_to_filenames,
                        input,
                        config.sounds_path.as_path(),
                        &sink,
                    )?;
                    read_chars.clear();
                }
                InputEventKind::Key(key) => {
                    match get_char(key) {
                        Some(ch) => read_chars.push(ch),
                        None => (),
                    };
                }
                _ => (),
            }
        }
    }
}
