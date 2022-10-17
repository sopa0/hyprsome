use std::fs::File;
use std::path::PathBuf;
use dirs::config_dir;

use crate::hyprland_ipc::monitor;
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct MonitorWorkspaces {
    monitor_name: String,
    workspaces: Vec<u32>,
}

#[derive(Serialize)]
struct TemplateData {
    monitor_workspaces: Vec<MonitorWorkspaces>,
    ws_numbers: Vec<u32>,
}

pub fn generate_hyprsome_config() {
    let monitors = monitor::get();
    let mut monitor_workspaces: Vec<MonitorWorkspaces> = vec![];
    monitors.into_iter().for_each(|m| {
        let first_workspace_number = match m.id {
            0 => 1,
            _ => format!("{}{}", m.id, 1).parse::<u32>().unwrap(),
        };

        monitor_workspaces.push(MonitorWorkspaces {
            monitor_name: m.name,
            workspaces: Vec::from_iter(first_workspace_number..first_workspace_number + 9),
        });
    });

    let ws_numbers = Vec::from_iter(1..10);
    let mut handlebars = Handlebars::new();

    // TODO: stop being stinky and handle errors
    _ = handlebars.register_template_string("hyprsome.conf", CONF_TEMPLATE);

    let data = TemplateData {
        monitor_workspaces,
        ws_numbers,
    };


    let mut relative_config_path = PathBuf::new();
    relative_config_path.push("hypr");
    relative_config_path.push("hyprsome");
    relative_config_path.set_extension("conf");

    let xdg_config_home = config_dir().unwrap();
    
    let mut absolute_config_path = xdg_config_home;
    absolute_config_path.push(relative_config_path);

    let mut output_file = match File::create(absolute_config_path) {
        Ok(output_file) => output_file,
        Err(e) => panic!("{:?}", e),
    };

    // TODO: stop being stinky and handle errors
    _= handlebars.render_to_write("hyprsome.conf", &data, &mut output_file);
}

const CONF_TEMPLATE: &str =
"{{#each monitor_workspaces}}
workspace={{this.monitor_name}},{{this.workspaces.0}}
{{/each}}

{{#each ws_numbers}}
bind=SUPER,{{this}},exec,hyprsome workspace {{this}}
{{/each}}

{{#each ws_numbers}}
bind=SUPER,{{this}},exec,hyprsome move {{this}}
{{/each}}

{{#each monitor_workspaces}}
    {{#each workspaces}}
wsbind={{this}},{{../monitor_name}}
    {{/each}}

{{/each}}";
