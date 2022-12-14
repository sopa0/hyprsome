use clap::{Parser, Subcommand, ValueEnum};

mod hyprland_ipc;
use hyprland_ipc::{client, monitor, option, workspace};

#[derive(Parser)]
#[command(name = "hyprsome")]
#[command(author = "sopa")]
#[command(version = "0.1.6")]
#[command(about = "Makes hyprland workspaces behave like awesome", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Focus { direction: Directions },
    Workspace { workspace_number: u64 },
    Move { workspace_number: u64 },
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Directions {
    L,
    R,
}

pub fn get_current_monitor() -> monitor::Monitor {
    return monitor::get()
        .into_iter()
        .find(|m| m.focused == true)
        .unwrap();
}

pub fn select_workspace(workspace_number: &u64) {
    let mon = get_current_monitor();
    match mon.id {
        0 => workspace::focus(workspace_number),
        _ => {
            workspace::focus(
                &format!("{}{}", mon.id, workspace_number)
                    .parse::<u64>()
                    .unwrap(),
            );
        }
    }
}

pub fn send_to_workspace(workspace_number: &u64) {
    let mon = get_current_monitor();
    match mon.id {
        0 => workspace::move_to(workspace_number),
        _ => {
            workspace::move_to(
                &format!("{}{}", mon.id, workspace_number)
                    .parse::<u64>()
                    .unwrap(),
            );
        }
    }
}

pub fn get_leftmost_client_for_monitor(mon_id: u64) -> client::Client {
    let clients = client::get();

    return clients
        .into_iter()
        .filter(|c| c.monitor == mon_id)
        .min_by_key(|c| c.at[0])
        .unwrap();
}

pub fn focus_left(aw: client::Client) {
    let mon = monitor::get_by_id(aw.monitor);

    if is_leftmost_monitor(&mon) && is_leftmost(&aw, &mon) {
        monitor::focus_by_id("1");

        return;
    }

    if is_leftmost(&aw, &mon) {
        monitor::focus_prev();

        return;
    }

    client::focus_by_direction("l");
}

pub fn focus_right(aw: client::Client) {
    let mon = monitor::get_by_id(aw.monitor);

    if is_rightmost_monitor(&mon) && is_rightmost(&aw, &mon) {
        monitor::focus_by_id("0");

        return;
    }

    if is_rightmost(&aw, &mon) {
        monitor::focus_next();

        return;
    }

    client::focus_by_direction("r");

    return;
}

pub fn is_leftmost(aw: &client::Client, mon: &monitor::Monitor) -> bool {
    let gaps = option::get_gaps();

    if aw.at[0] - gaps == mon.x {
        return true;
    }

    return false;
}

pub fn is_rightmost(aw: &client::Client, mon: &monitor::Monitor) -> bool {
    let gaps = option::get_gaps();

    if mon.real_width() + mon.x - gaps == aw.size[0] + aw.at[0] {
        return true;
    }

    return false;
}

pub fn is_rightmost_monitor(mon: &monitor::Monitor) -> bool {
    let monitors = monitor::get();
    let max = monitors.into_iter().max_by_key(|m| m.x).unwrap();
    if max.x == mon.x {
        return true;
    }
    return false;
}

pub fn is_leftmost_monitor(mon: &monitor::Monitor) -> bool {
    let monitors = monitor::get();
    let min = monitors.into_iter().min_by_key(|m| m.x).unwrap();
    if min.x == mon.x {
        return true;
    }
    return false;
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Focus { direction } => match direction {
            Directions::L => {
                let aw_res = client::get_active();

                match aw_res {
                    Ok(aw) => focus_left(aw),
                    Err(_e) => monitor::focus_prev(),
                };
            }
            Directions::R => {
                let aw_res = client::get_active();

                match aw_res {
                    Ok(aw) => focus_right(aw),
                    Err(_e) => monitor::focus_next(),
                };
            }
        },
        Commands::Workspace { workspace_number } => {
            select_workspace(workspace_number);
        }
        Commands::Move { workspace_number } => {
            send_to_workspace(workspace_number);
        }
    }
}
