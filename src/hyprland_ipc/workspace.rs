// TODO: change this file to hyprland-rs
const WORKSPACE: &str = "workspace";
const DISPATCH: &str = "dispatch";
const MOVETOWORKSPACESILENT: &str = "movetoworkspacesilent";
const MOVETOWORKSPACE: &str = "movetoworkspace";

pub fn focus(workspace_number: &u64) {
    let _ = super::send_message(DISPATCH, vec![WORKSPACE, &workspace_number.to_string()]);
}

pub fn move_to(workspace_number: &u64) {
    super::send_message(
        DISPATCH,
        vec![MOVETOWORKSPACESILENT, &workspace_number.to_string()],
    );
}

pub fn move_focus(workspace_number: &u64) {
    super::send_message(
        DISPATCH,
        vec![MOVETOWORKSPACE, &workspace_number.to_string()],
    );
}
