# Hyprsome
Hyprsome is a binary that interacts with Hyprland's Unix socket to make workspaces behave similarly to AwesomeWM in a multi-monitor setup.

If you're focused on a monitor and press SUPER+[1-9], you'll only switch to the workspaces that are bound to that monitor.

It is inspired by Swaysome, which does a similar thing for Sway.

# Installation
`
cargo install hyprsome
`

# Usage
Once the binary is installed, you can modify your ~/.config/hypr/hyprland.conf to accomodate it.
Here is an example of a dual monitor setup:

```
monitor=DP-1,1920x1080@60,0x0,1.33
monitor=DP-1,transform,1
workspace=DP-1,1
monitor=HDMI-A-1,3440x1440@100,813x0,1
workspace=HDMI-A-1,11
```

Most noteworthy thing here is the 'workspace' keyword that I use to bind a default workspace for each monitor.


Then you can bind workspaces to your different monitors.

It is very important that you bind your workspaces in order.

Check the results of `hyprctl monitors`. Bind workspaces from 1 to 9 on your monitor that has 0 as an id.

Then just bind workspaces by prefixing numbers by the id of the monitor they're bound to.

Here, HDMI-A-1's id is 1, so I bind workspaces from 11 to 19 to it.

```
  workspace=1,monitor:DP-1
  workspace=2,monitor:DP-1
  workspace=3,monitor:DP-1
  workspace=4,monitor:DP-1
  workspace=5,monitor:DP-1

  workspace=11,monitor:HDMI-A-1
  workspace=12,monitor:HDMI-A-1
  workspace=13,monitor:HDMI-A-1
  workspace=14,monitor:HDMI-A-1
  workspace=15,monitor:HDMI-A-1
```

Then it's just a matter of making sure your regular workspace keybinds call hyprsome.

```
bind=SUPER,1,exec,hyprsome workspace 1
bind=SUPER,2,exec,hyprsome workspace 2
bind=SUPER,3,exec,hyprsome workspace 3
bind=SUPER,4,exec,hyprsome workspace 4
bind=SUPER,5,exec,hyprsome workspace 5

bind=SUPERSHIFT,1,exec,hyprsome move 1
bind=SUPERSHIFT,2,exec,hyprsome move 2
bind=SUPERSHIFT,3,exec,hyprsome move 3
bind=SUPERSHIFT,4,exec,hyprsome move 4
bind=SUPERSHIFT,5,exec,hyprsome move 5

```

# Limitations
This is alpha software and my first program in Rust, bugs are bound to happen but nothing that will break your system.

Some features are most likely missing.

You can only have 9 workspaces per monitor as of now.

I haven't worked on supporting monitor hot-plug at all. It may work but it's unlikely.


# Alternatives

https://github.com/Duckonaut/split-monitor-workspaces leverages the plugin system, is more feature rich and seems better maintained
