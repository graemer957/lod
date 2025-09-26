# Laptop or Desktop (aka `lod`)

## What

This is a small utility app for macOS that lives in the menu bar in the status area. Its purpose is to provide a quick way to switch settings depending on whether your laptop is docked or not.

For example, when in desktop mode I prefer my mouse not to use natural scrolling (what can I say I've been using it this way since dot) and for the dock to be shown. However, in laptop mode, I prefer natural scrolling and having my dock hidden.

I also use this for keeping my machine awake using the `caffeinate` built-in utility.

## How

Sadly, in the Apple ecosystem, it seems pretty difficult to change system settings. I looked into various approaches (eg, using `defaults`), but the one which appeared to give the most flexibility was to use AppleScript.

## Why

A personal experiment in writing a GUI app in Rust. I do not anticipate it will be useful to many. **This is intentionally a lightweight utility focused on learning rather than production robustness.** I fully hope there are easier ways to perform this kind of automation because working with AppleScript is not exactly fun. Feel free to get in touch and let me know those easier ways ;-)

### Learning

- Writing a macOS app in Rust, was not 'pleasant' given the heritage (Objective-C)
- [system_status_bar_macos](https://github.com/amachang/system_status_bar_macos) seemed like a good fit, but needed to extend it. Raised a few PRs:
    - [[1/2] Extend `StatusItem` and `MenuItem` API](https://github.com/amachang/system_status_bar_macos/pull/1)
    - [[2/2] Run rustfmt over the whole project](https://github.com/amachang/system_status_bar_macos/pull/2)
- Initially, the crate author seemed willing to incorporate, but they are left unmerged as of 27/04/2025 😔
- Forked [graemer957/system_status_bar_macos](https://github.com/graemer957/system_status_bar_macos)
    - Extended `StatusItem` and `MenuItem`
    - Formatted using `rustfmt`
    - Substantially reduced build times (need to write up a blog post on this!)
    - Modified the `event_loop!` so it does not waste CPU cycles
- The AppleScripts I wrote ended up being flakey across macOS version upgrades

## Installing

Simply use cargo install along with the path to this repo using the protocol you prefer, eg:

```fish
cargo install --git ssh://git@github.com/graemer957/lod --locked
```

## Updating

Use the same command from `Installing` to upgrade. I have yet to find a way to perform updates on all my command line utilities installed via `cargo`.

## Running

Once installed, execute `lod` on the command line. I have some ideas on how to make the UX better, see `Future Ideas` below.

## Configuring

In `~/.config/lod/config.toml` you can set:
```toml
desktop_applescript = "<AppleScript to run when switching into Desktop mode>"
laptop_applescript = "<AppleScript to run when switching into Laptop mode>"

# Optional
# caffeinate_app = "<Path to custom binary for keeping machine wake>"
# caffeinate_options = "<Options to pass to custom binary>"
```

## Future ideas

An incomplete list of ideas which could make this project more useful. I may get to them one day, but feel free to raise a PR if any are handy for you sooner :-)

- [ ] Launch script to start on boot
- [ ] Logging into the unified logging system
- [ ] Observability, eg crash reporting and maybe anonymous usage information
- [ ] Add tooltips or colour to the menu item(s) (this depends on enhancing [system_status_bar_macos](https://github.com/amachang/system_status_bar_macos)
and so far the author has not merged any of my other PRs
- [ ] Triggers based on hotkeys or hardware events (insertion of mouse triggers desktop mode)
- [ ] Automatically `caffeinate` depending on power state
- [ ] Improve automated test coverage
