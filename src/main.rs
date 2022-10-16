use clap::{Parser, Subcommand, ValueEnum};

mod hyprland_ipc;
use hyprland_ipc::{
    focus_mon, focus_next_mon, focus_prev_mon, focus_workspace, get_active_window, get_clients,
    get_gaps, get_monitor_by_id, get_monitors, move_focus, move_to_workspace, Client, Monitor,
};

#[derive(Parser)]
#[command(name = "Hyprsome")]
#[command(author = "sopa")]
#[command(version = "0.1")]
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

pub fn get_current_monitor() -> Monitor {
    return get_monitors()
        .into_iter()
        .find(|m| m.focused == true)
        .unwrap();
}

pub fn select_workspace(workspace_number: &u64) {
    let mon = get_current_monitor();
    match mon.id {
        0 => focus_workspace(workspace_number),
        _ => {
            focus_workspace(
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
        0 => move_to_workspace(workspace_number),
        _ => {
            move_to_workspace(
                &format!("{}{}", mon.id, workspace_number)
                    .parse::<u64>()
                    .unwrap(),
            );
        }
    }
}

pub fn get_leftmost_client_for_monitor(mon_id: u64) -> Client {
    let clients = get_clients();

    return clients
        .into_iter()
        .filter(|c| c.monitor == mon_id)
        .min_by_key(|c| c.at[0])
        .unwrap();
}

pub fn focus_left(aw: Client) {
    let mon = get_monitor_by_id(aw.monitor);

    if is_leftmost_monitor(&mon) && is_leftmost(&aw, &mon) {
        focus_mon("1");

        return;
    }

    if is_leftmost(&aw, &mon) {
        focus_prev_mon();

        return;
    }

    move_focus("l");
}

pub fn focus_right(aw: Client) {
    let mon = get_monitor_by_id(aw.monitor);

    if is_rightmost_monitor(&mon) && is_rightmost(&aw, &mon) {
        focus_mon("0");

        return;
    }

    if is_rightmost(&aw, &mon) {
        focus_next_mon();

        return;
    }

    move_focus("r");

    return;
}

pub fn is_leftmost(aw: &Client, mon: &Monitor) -> bool {
    let gaps = get_gaps();

    if aw.at[0] - gaps == mon.x {
        return true;
    }

    return false;
}

pub fn is_rightmost(aw: &Client, mon: &Monitor) -> bool {
    let gaps = get_gaps();

    if mon.real_width() + mon.x - gaps == aw.size[0] + aw.at[0] {
        return true;
    }

    return false;
}

pub fn is_rightmost_monitor(mon: &Monitor) -> bool {
    let monitors = get_monitors();
    let max = monitors.into_iter().max_by_key(|m| m.x).unwrap();
    if max.x == mon.x {
        return true;
    }
    return false;
}

pub fn is_leftmost_monitor(mon: &Monitor) -> bool {
    let monitors = get_monitors();
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
                let aw_res = get_active_window();

                match aw_res {
                    Ok(aw) => focus_left(aw),
                    Err(_e) => focus_prev_mon(),
                };
            }
            Directions::R => {
                let aw_res = get_active_window();

                match aw_res {
                    Ok(aw) => focus_right(aw),
                    Err(_e) => focus_next_mon(),
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
