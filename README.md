
# Laptop or Desktop (aka `lod`)

This is a small utility app for macOS that lives in the menu bar in the status area. It's purpose
is to provide a quick way to switch settings depending on whether your laptop is docked or not.

For example, when in desktop mode I prefer my mouse not to use natural scrolling (what can I say
I've been using it this way since .) and for the dock to be shown. However, in laptop mode, I
prefer natural scrolling and having my dock hidden.

## How

Sadly, in the Apple ecosystem, it seems pretty difficult to change system settings. I looked into
various approaches (eg, using `defaults`), but the one which appeared to give the most flexibility
was to use AppleScript.

## Why

A personal experiment in writing macOS apps in Rust. I do not anticipate it will be useful to many.
I fully hope there are easier ways to perform this kind of automation because working
with AppleScript is not exactly fun. Feel free to get in touch and let me know those easier ways ;-)

## Installing

Simply use cargo install along with the path to this repo using the protocol you prefer, eg:

```fish
cargo install --git ssh://git@github.com/graemer957/lod
```

## Updating

Use the same command from `Installing` to upgrade. I have yet to find a way to perform updates
on all my command line utilities installed via `cargo`.

## Running

Once installed, execute `lod` on the command line. I have some ideas on how to make the UX
better, see `Future Ideas` below.

## Future ideas

An incomplete list of ideas which could make this project more useful. I may get to them one day,
but feel free to raise a PR if any are handy for you sooner :-)

- [ ] Launch script to start on boot
- [ ] Logging into the unified logging system
- [ ] Observability, eg crash reporting and maybe anonymous usage information
- [ ] Entity diagram
- [ ] CI/CD pipeline
- [ ] Add tooltip's to the menu item (this depends on enhancing [system_status_bar_macos](https://github.com/amachang/system_status_bar_macos)
and so far the author has not merged any of my other PRs
- [ ] Triggers based on hotkeys or hardware events (insertion of mouse triggers desktop mode)

