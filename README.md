# aim2go.xyz 

## Day1 
Started with ratatui simple-async template
added logo
added clap menu
added auto clicker module

# Day2
auto-clicker module complete
auto-clicker module config

## TODO:
- [ ] (modules/autoclicker.rs) auto_clicker_start()
- [ ] (modules/autoclicker.rs) auto_clicker_stop()
- [ ] (main.rs) match autoclicker commands
- [ ] (app.rs) handle modes (Execute/Config/Forum/Help)
- [ ] (ui.rs) Execute mode
- [ ] (ui.rs) Config mode
- [ ] (ui.rs) Forum mode
- [ ] (ui.rs) Help mode

## Execute mode
Executes Config (Change ui later)

# Config mode
toml editor 

## Website
- Forum for members to share configurations. 
- Create/Login account page. 
- Stripe page

## Price
- $4.99/month

## Client features
- Cross Platform (Linux and Windows)
- Visual/Input only hacks 
- Tui/Cli interface
- Pause on menus


## Cheats
- [ ] AutoClicker
- [ ] AimAssist
- [ ] SlientAimBot (AimAssist and AutoClicker)
- [ ] AutoDodge
- [ ] AutoBlock
- [ ] Macros
- [ ] ESP
- [ ] AFKBot

## Crates/Templates 

### [enigo](https://crates.io/crates/enigo)
- Purpose: Simulate keyboard and mouse events.

### [tch](https://crates.io/crates/tch)
- Purpose: Rust wrappers for PyTorch's C++ API.

### [xcap](https://crates.io/crates/xcap)
- Purpose: Cross-platform screen capture library.

### [ratatui-component](https://github.com/ratatui/templates/tree/main/component)
- Template: ratatui component template

## how aim2go works
each module/hack is ran in its own process so they can be individually turned off at runtime
