#![allow(dead_code)]
use std::collections::HashMap;
use std::env;
use std::process::Command;

fn get_xrandr_out() -> String {
    let xrandr_child = Command::new("xrandr")
        .arg("--listmonitors")
        .output()
        .expect("Failed to start xrandr process");

    String::from_utf8(xrandr_child.stdout).unwrap()
}

fn get_monitors() -> Vec<String> {
    get_xrandr_out()
        .lines()
        .skip(1)
        .map(|line| {
            line.split_whitespace()
                .last()
                .unwrap_or_default()
                .to_string()
        })
        .collect()
}

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn from(v: &[u32]) -> Self {
        Position { x: v[0], y: v[1] }
    }
}

fn get_monitor_positions() -> HashMap<String, Position> {
    let mut position_hm = HashMap::new();

    for line in get_xrandr_out().lines().skip(1) {
        let monitor_name = line
            .split_whitespace()
            .last()
            .unwrap_or_default()
            .to_string();

        let monitor_position_str: Vec<&str> = line.split('+').skip(2).take(2).collect();

        let monitor_positions: Vec<u32> = monitor_position_str
            .iter()
            .map(|el| el.parse::<u32>().unwrap_or(0))
            .collect();

        position_hm.insert(monitor_name, Position::from(&monitor_positions));
    }

    position_hm
}

fn extend_displays() {
    let monitors = get_monitors();

    let mut args: Vec<&str> = Vec::new();

    for (i, monitor) in monitors.iter().enumerate() {
        args.push("--output");
        args.push(monitor);
        args.push("--auto");
        if i != 0 {
            args.push("--right-of");
            args.push(monitors[i - 1].as_str());
        }
    }

    Command::new("xrandr")
        .args(args)
        .spawn()
        .expect("Failed to execute extend command.");
}

fn switch_displays() {
    let positions = get_monitor_positions();
    let mut monitors: Vec<&String> = positions.keys().collect();
    monitors.sort_by_key(|key| match positions.get(key.to_owned()) {
        Some(p) => p.x,
        None => 0,
    });

    let mut args: Vec<&str> = Vec::new();

    for (i, monitor) in monitors.iter().enumerate() {
        args.push("--output");
        args.push(monitor);
        args.push("--auto");
        if i != 0 {
            args.push("--left-of");
            args.push(monitors[i - 1].as_str());
        }
    }

    Command::new("xrandr")
        .args(args)
        .spawn()
        .expect("Failed to execute switch command.");
}

fn mirror_displays() {
    let monitors = get_monitors();

    let mut args: Vec<&str> = Vec::new();

    for monitor in monitors.iter() {
        args.push("--output");
        args.push(monitor);
        args.push("--auto");
    }

    Command::new("xrandr")
        .args(args)
        .arg("--same-as")
        .arg(monitors[0].as_str())
        .spawn()
        .expect("Failed to execute mirror command.");
}

fn help() {
    println!(
        "Usage:
xrandrs <command>

Commands:
    extend
    list
    switch
    mirror"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments
        1 => {
            println!("No commands given.")
        }
        2 => match args[1].as_str() {
            "extend" => extend_displays(),
            "list" => todo!(),
            "switch" => switch_displays(),
            "mirror" => mirror_displays(),
            _ => help(),
        },
        _ => help(),
    }
}
