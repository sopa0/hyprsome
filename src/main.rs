use clap::{Parser, Subcommand, ValueEnum};

mod hyprland_ipc;
use hyprland::{
    data::{Client, Monitor, Transforms},
    dispatch::{Direction, Dispatch, DispatchType, WorkspaceIdentifier, MonitorIdentifier}, keyword::{Keyword, OptionValue},
};
use hyprland_ipc::{client, monitor, option, workspace};

#[derive(Parser)]
#[command(name = "hyprsome")]
#[command(author = "sopa0")]
#[command(version = "0.1.11")]
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
    Movefocus { workspace_number: u64 },
    Bindworkspaces,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Directions {
    L,
    R,
    U,
    D,
}

pub trait MonitorDimensions {
    fn real_width(&self) -> f32;
    fn real_height(&self) -> f32;
}

impl MonitorDimensions for Monitor {
    fn real_width(&self) -> f32 {
        match self.transform {
            Transforms::Normal
            | Transforms::Normal180
            | Transforms::Flipped
            | Transforms::Flipped180 => self.width as f32 / self.scale,
            Transforms::Normal90 | Transforms::Normal270 | Transforms::Flipped90 => {
                self.height as f32 / self.scale
            }
            _ => self.width as f32,
        }
    }

    fn real_height(&self) -> f32 {
        match self.transform {
            Transforms::Normal
            | Transforms::Flipped
            | Transforms::Normal180
            | Transforms::Flipped180 => self.height as f32 / self.scale,
            Transforms::Normal90 | Transforms::Normal270 | Transforms::Flipped90 => {
                self.width as f32 / self.scale
            }
            _ => self.height as f32,
        }
    }
}

pub fn get_current_monitor() -> Monitor {
    monitor::get().find(|m| m.focused).unwrap()
}

pub fn bind_workspaces() {
    monitor::get().for_each(|mon| {
        let monitor_id = mon.id;
        let name = mon.name;
        for i in 1..=9 {
            let workspace_number = i + (monitor_id * 10);
            let workspace_config = format!("{workspace_number},monitor:{name},default:{}", i == 1);
            Keyword::set("workspace", OptionValue::String(workspace_config)).unwrap();
        }
        let _ = Dispatch::call(DispatchType::MoveWorkspaceToMonitor(WorkspaceIdentifier::Id((monitor_id + 1) as i32), MonitorIdentifier::Id((monitor_id / 10) as u8)));
    })
}

//TODO: refactor this nonsense
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

//TODO: refactor this nonsense
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

//TODO: refactor this nonsense
pub fn movefocus(workspace_number: &u64) {
    let mon = get_current_monitor();
    match mon.id {
        0 => workspace::move_focus(workspace_number),
        _ => {
            workspace::move_focus(
                &format!("{}{}", mon.id, workspace_number)
                    .parse::<u64>()
                    .unwrap(),
            );
        }
    }
}

pub fn get_leftmost_client_for_monitor(mon_id: i16) -> Client {
    let clients = client::get();

    clients
        .into_iter()
        .filter(|c| c.monitor == mon_id)
        .min_by_key(|c| c.at.0)
        .unwrap()
}

pub fn focus_left(aw: Client) {
    let mon = monitor::get_by_id(aw.monitor);
    let is_leftmost_client = is_leftmost_client(&aw, &mon);

    if is_leftmost_monitor(&mon) && is_leftmost_client {
        return;
    }

    if is_leftmost_client {
        monitor::focus_left();

        return;
    }

    client::focus_by_direction(Direction::Left);
}

pub fn focus_right(aw: Client) {
    let mon = monitor::get_by_id(aw.monitor);

    if is_rightmost_monitor(&mon) && is_rightmost_client(&aw, &mon) {
        return;
    }

    if is_rightmost_client(&aw, &mon) {
        monitor::focus_right();

        return;
    }

    client::focus_by_direction(Direction::Right);
}

pub fn focus_up(aw: Client) {
    let mon = monitor::get_by_id(aw.monitor);
    let is_top_client = is_top_client(&aw, &mon);

    if is_top_monitor(&mon) && is_top_client {
        return;
    }

    if is_top_client {
        monitor::focus_up();

        return;
    }

    client::focus_by_direction(Direction::Up);
}

pub fn focus_down(aw: Client) {
    let mon = monitor::get_by_id(aw.monitor);
    let is_bottom_client = is_bottom_client(&aw, &mon);

    if is_bottom_monitor(&mon) && is_bottom_client {
        return;
    }

    if is_bottom_client {
        monitor::focus_down();

        return;
    }

    client::focus_by_direction(Direction::Down);
}

pub fn is_leftmost_client(aw: &Client, mon: &Monitor) -> bool {
    let gaps = option::get_gaps();

    if (aw.at.0 - gaps) as i32 == mon.x {
        return true;
    }

    false
}

pub fn is_rightmost_client(aw: &Client, mon: &Monitor) -> bool {
    let gaps = option::get_gaps();

    if mon.real_width() + mon.x as f32 - gaps as f32 == aw.size.0 as f32 + aw.at.0 as f32 {
        return true;
    }

    false
}

pub fn is_top_client(aw: &Client, mon: &Monitor) -> bool {
    let gaps = option::get_gaps();

    if mon.y + (gaps as i32) + (mon.reserved.1 as i32) == (aw.at.1 as i32) {
        return true;
    }

    false
}

pub fn is_bottom_client(aw: &Client, mon: &Monitor) -> bool {
    let gaps = option::get_gaps();

    if mon.real_height() + mon.y as f32 - gaps as f32 - mon.reserved.1 as f32
        == aw.size.1 as f32 + gaps as f32
    {
        return true;
    }

    false
}

pub fn is_rightmost_monitor(mon: &Monitor) -> bool {
    let monitors = monitor::get();
    let max = monitors.into_iter().max_by_key(|m| m.x).unwrap();
    if max.x == mon.x {
        return true;
    }
    false
}

pub fn is_leftmost_monitor(mon: &Monitor) -> bool {
    let monitors = monitor::get();
    let min = monitors.into_iter().min_by_key(|m| m.x).unwrap();
    if min.x == mon.x {
        return true;
    }
    false
}

pub fn is_top_monitor(mon: &Monitor) -> bool {
    let monitors = monitor::get();
    let min = monitors.into_iter().min_by_key(|m| m.y).unwrap();
    if min.y == mon.y {
        return true;
    }
    false
}

pub fn is_bottom_monitor(mon: &Monitor) -> bool {
    let monitors = monitor::get();
    let max = monitors.into_iter().max_by_key(|m| m.y).unwrap();
    if max.y == mon.y {
        return true;
    }
    false
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Focus { direction } => match direction {
            Directions::L => {
                let aw = client::get_active();

                match aw {
                    Some(aw) => focus_left(aw),
                    None => monitor::focus_left(),
                };
            }
            Directions::R => {
                let aw = client::get_active();

                match aw {
                    Some(aw) => focus_right(aw),
                    None => monitor::focus_right(),
                };
            }
            Directions::U => {
                let aw = client::get_active();

                match aw {
                    Some(aw) => focus_up(aw),
                    None => monitor::focus_up(),
                };
            }
            Directions::D => {
                let aw = client::get_active();

                match aw {
                    Some(aw) => focus_down(aw),
                    None => monitor::focus_down(),
                };
            }
        },
        Commands::Workspace { workspace_number } => {
            select_workspace(workspace_number);
        }
        Commands::Move { workspace_number } => {
            send_to_workspace(workspace_number);
        }
        Commands::Movefocus { workspace_number } => {
            movefocus(workspace_number);
        }
        Commands::Bindworkspaces => {
            bind_workspaces();
        },
    }
}
