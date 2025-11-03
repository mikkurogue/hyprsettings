# Hyprconfig - The hyprland gui settings tool for idots

Basically just a small utility to on the fly edit the hyprland coniguration settings that are defined.
Some people are just bad at reading the docs, and some are bad at understanding words (me), so I wanted 
to create a tool that will allow users to find, re-define or change their settings from a gui application.

I am a firm beliver that if Linux needs more wider adoption, and especially Hyprland as well, we should
have an ecosystem that is also "noob" or "normie" friendly.

## Status

Project is very much still in progress, I'm working on the basics and writing down what I still need to figure out.

Currently I need to figure out how to read (potentially) all `~/.config/hypr/*.conf` files to figure out current values,
and to have a nicer way to do so.

Another idea I have is to create a `hyprconf.conf` inside the `hypr` configuration directory, and add it to the bottom of `hyprland.conf` as a source
file so it can override all existing settings. This is especially handy for distributions like `Omarchy` or other `Hyprland dot file distributors` like the HyDE project etc.

Feedback and contributions are welcome :)
