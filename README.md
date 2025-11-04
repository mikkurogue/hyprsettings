# Hyprconfig

![build](https://github.com/mikkurogue/hyprconfig/actions/workflows/rust.yml/badge.svg)

A GUI configuration tool for Hyprland designed to make setup and configuration easier for everyone.

## What is Hyprconfig?

Hyprconfig is a graphical user interface tool that helps you configure Hyprland without having to manually edit configuration files. It's designed specifically for:

- **New Linux/Hyprland users** who are still learning their way around
- **Users who prefer GUIs** over editing text configuration files
- **Anyone who wants a pain-free setup** for monitors, inputs, and other core settings

Whether you're not super tech-savvy or simply want to configure your main Hyprland settings quickly and easily, Hyprconfig provides an intuitive interface to get your system set up without diving into documentation or syntax.

## Overrides
This project (on first run) will create and append a new `conf-overrides.conf` to your hyprland config.
This will by itself also then write all overrides into this new file.

This file is meant to not be very organized or "readable" as its only job is to exist as a configuration file.
Also the only easy configurations that are settable for now are ones that provide us with unique identifiers until i can figure out a good solution.

I.e. monitors, as we can fetch monitors with `hyprctl monitors all` and get every connected monitor
For inputs like mouse (sensitivity) and keyboard (layout/locale) is easy enough to also do

## Status

This project is actively in development. The basics are being worked on and core functionality is being implemented.

## Contributing

**Contributions are welcome!** Whether you want to report bugs, suggest features, improve documentation, or submit code, your help is appreciated. Feel free to open issues or pull requests.
