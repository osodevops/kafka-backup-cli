-- WezTerm profile for kafka-backup-monitor screen recording
-- Usage: wezterm --config-file assets/terminal-profiles/wezterm.lua
local wezterm = require("wezterm")
local config = wezterm.config_builder()

config.font = wezterm.font("JetBrains Mono", { weight = "Regular" })
config.font_size = 16.0
config.line_height = 1.1

config.initial_cols = 120
config.initial_rows = 40

config.window_background_opacity = 1.0
config.colors = {
    background = "#0F172A",
    foreground = "#F8FAFC",
    cursor_bg = "#F8FAFC",
    cursor_fg = "#0F172A",
    selection_bg = "#334155",
    selection_fg = "#F8FAFC",

    ansi = {
        "#0F172A", -- black
        "#EF4444", -- red
        "#10B981", -- green
        "#F59E0B", -- yellow
        "#2563EB", -- blue
        "#8B5CF6", -- magenta
        "#06B6D4", -- cyan
        "#F8FAFC", -- white
    },
    brights = {
        "#64748B", -- bright black
        "#EF4444", -- bright red
        "#10B981", -- bright green
        "#F59E0B", -- bright yellow
        "#38BDF8", -- bright blue
        "#8B5CF6", -- bright magenta
        "#06B6D4", -- bright cyan
        "#F8FAFC", -- bright white
    },
}

config.window_padding = {
    left = 20,
    right = 20,
    top = 20,
    bottom = 20,
}

config.enable_tab_bar = false
config.window_decorations = "RESIZE"

return config
