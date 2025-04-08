pub const STAR1: &str = r#"format = """
[](#9A348E)\
$os\
$username\
[](bg:#DA627D fg:#9A348E)\
$directory\
[](fg:#DA627D bg:#FCA17D)\
$git_branch\
$git_status\
[](fg:#FCA17D bg:#86BBD8)\
$c\
$elixir\
$elm\
$golang\
$gradle\
$haskell\
$java\
$julia\
$nodejs\
$nim\
$rust\
$scala\
[](fg:#86BBD8 bg:#06969A)\
$docker_context\
[](fg:#06969A bg:#33658A)\
$time\
[ ](fg:#33658A)\
"""

# Disable the blank line at the start of the prompt
# add_newline = false

# You can also replace your username with a neat symbol like   or disable this
# and use the os module below
[username]
show_always = true
style_user = "bg:#9A348E"
style_root = "bg:#9A348E"
format = '[$user ]($style)'
disabled = false

# An alternative to the username module which displays a symbol that
# represents the current operating system
[os]
style = "bg:#9A348E"
disabled = true # Disabled by default

[directory]
style = "bg:#DA627D"
format = "[ $path ]($style)"
truncation_length = 3
truncation_symbol = "…/"

# Here is how you can shorten some long paths by text replacement
# similar to mapped_locations in Oh My Posh:
[directory.substitutions]
"Documents" = "󰈙 "
"Downloads" = " "
"Music" = " "
"Pictures" = " "
# Keep in mind that the order matters. For example:
# "Important Documents" = " 󰈙 "
# will not be replaced, because "Documents" was already substituted before.
# So either put "Important Documents" before "Documents" or use the substituted version:
# "Important 󰈙 " = " 󰈙 "

[c]
symbol = " "
style = "bg:#86BBD8"
format = '[ $symbol ($version) ]($style)'

[docker_context]
symbol = " "
style = "bg:#06969A"
format = '[ $symbol $context ]($style)'

[elixir]
symbol = " "
style = "bg:#86BBD8"
format = '[ $symbol ($version) ]($style)'

[elm]
symbol = " "
style = "bg:#86BBD8"
format = '[ $symbol ($version) ]($style)'

[git_branch]
symbol = ""
style = "bg:#FCA17D"
format = '[ $symbol $branch ]($style)'

[git_status]
style = "bg:#FCA17D"
format = '[$all_status$ahead_behind ]($style)'

[golang]
symbol = " "
style = "bg:#86BBD8"
format = '[ $symbol ($version) ]($style)'

[gradle]
style = "bg:#86BBD8"
format = '[ $symbol ($version) ]($style)'

[haskell]
symbol = " "
style = "bg:#86BBD8"
format = '[ $symbol ($version) ]($style)'

[java]
symbol = " "
style = "bg:#86BBD8"
format = '[ $symbol ($version) ]($style)'

[julia]
symbol = " "
style = "bg:#86BBD8"
format = '[ $symbol ($version) ]($style)'

[nodejs]
symbol = ""
style = "bg:#86BBD8"
format = '[ $symbol ($version) ]($style)'

[nim]
symbol = "󰆥 "
style = "bg:#86BBD8"
format = '[ $symbol ($version) ]($style)'

[rust]
symbol = ""
style = "bg:#86BBD8"
format = '[ $symbol ($version) ]($style)'

[scala]
symbol = " "
style = "bg:#86BBD8"
format = '[ $symbol ($version) ]($style)'

[time]
disabled = false
time_format = "%R" # Hour:Minute Format
style = "bg:#33658A"
format = '[ ♥ $time ]($style)'

"#;

pub const STAR2: &str = r#"[character]
success_symbol = "[>](bold green)"
error_symbol = "[x](bold red)"
vimcmd_symbol = "[<](bold green)"

[git_commit]
tag_symbol = " tag "

[git_status]
ahead = ">"
behind = "<"
diverged = "<>"
renamed = "r"
deleted = "x"

[aws]
symbol = "aws "

[azure]
symbol = "az "

[buf]
symbol = "buf "

[bun]
symbol = "bun "

[c]
symbol = "C "

[cobol]
symbol = "cobol "

[conda]
symbol = "conda "

[crystal]
symbol = "cr "

[cmake]
symbol = "cmake "

[daml]
symbol = "daml "

[dart]
symbol = "dart "

[deno]
symbol = "deno "

[dotnet]
symbol = ".NET "

[directory]
read_only = " ro"

[docker_context]
symbol = "docker "

[elixir]
symbol = "exs "

[elm]
symbol = "elm "

[fennel]
symbol = "fnl "

[fossil_branch]
symbol = "fossil "

[gcloud]
symbol = "gcp "

[git_branch]
symbol = "git "

[gleam]
symbol = "gleam "

[golang]
symbol = "go "

[gradle]
symbol = "gradle "

[guix_shell]
symbol = "guix "

[hg_branch]
symbol = "hg "

[java]
symbol = "java "

[julia]
symbol = "jl "

[kotlin]
symbol = "kt "

[lua]
symbol = "lua "

[nodejs]
symbol = "nodejs "

[memory_usage]
symbol = "memory "

[meson]
symbol = "meson "

[nats]
symbol = "nats "

[nim]
symbol = "nim "

[nix_shell]
symbol = "nix "

[ocaml]
symbol = "ml "

[opa]
symbol = "opa "

[os.symbols]
AIX = "aix "
Alpaquita = "alq "
AlmaLinux = "alma "
Alpine = "alp "
Amazon = "amz "
Android = "andr "
Arch = "rch "
Artix = "atx "
CachyOS = "cach "
CentOS = "cent "
Debian = "deb "
DragonFly = "dfbsd "
Emscripten = "emsc "
EndeavourOS = "ndev "
Fedora = "fed "
FreeBSD = "fbsd "
Garuda = "garu "
Gentoo = "gent "
HardenedBSD = "hbsd "
Illumos = "lum "
Kali = "kali "
Linux = "lnx "
Mabox = "mbox "
Macos = "mac "
Manjaro = "mjo "
Mariner = "mrn "
MidnightBSD = "mid "
Mint = "mint "
NetBSD = "nbsd "
NixOS = "nix "
Nobara = "nbra "
OpenBSD = "obsd "
OpenCloudOS = "ocos "
openEuler = "oeul "
openSUSE = "osuse "
OracleLinux = "orac "
Pop = "pop "
Raspbian = "rasp "
Redhat = "rhl "
RedHatEnterprise = "rhel "
RockyLinux = "rky "
Redox = "redox "
Solus = "sol "
SUSE = "suse "
Ubuntu = "ubnt "
Ultramarine = "ultm "
Unknown = "unk "
Uos = "uos "
Void = "void "
Windows = "win "

[package]
symbol = "pkg "

[perl]
symbol = "pl "

[php]
symbol = "php "

[pijul_channel]
symbol = "pijul "

[pulumi]
symbol = "pulumi "

[purescript]
symbol = "purs "

[python]
symbol = "py "

[quarto]
symbol = "quarto "

[raku]
symbol = "raku "

[ruby]
symbol = "rb "

[rust]
symbol = "rs "

[scala]
symbol = "scala "

[spack]
symbol = "spack "

[solidity]
symbol = "solidity "

[status]
symbol = "[x](bold red) "

[sudo]
symbol = "sudo "

[swift]
symbol = "swift "

[typst]
symbol = "typst "

[terraform]
symbol = "terraform "

[zig]
symbol = "zig "

"#;

pub const WEZ1: &str = r#"local wezterm = require 'wezterm'
local config = {}

if wezterm.config_builder then
  config = wezterm.config_builder()
end

config.default_prog = { 'nu' }

config.font = wezterm.font 'Fira Code'

config.font_size = 11

config.color_scheme = 'Catppuccin Mocha'

config.window_background_opacity = 0.66

config.enable_tab_bar = false

config.window_background_gradient = {
  interpolation = 'Linear',

  orientation = 'Vertical',

  blend = 'Rgb',

  colors = {
    '#11111b',
    '#181825',
  },
}

config.use_fancy_tab_bar = false

return config

"#;