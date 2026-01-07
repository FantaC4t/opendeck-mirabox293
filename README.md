![Plugin Icon](assets/icon.png)

# OpenDeck Mirabox 293 Plugin

An unofficial plugin for Mirabox 293 (15-key, 3x5 layout)

## OpenDeck version

Requires OpenDeck 2.5.0 or newer

## Supported devices

- Mirabox 293 (5500:1001)

## Platform support

- Linux: Primary target
- Mac: Best effort, things may break
- Windows: Best effort, things may break

## Installation

1. Download an archive from [releases](https://github.com/FantaC4t/opendeck-mirabox293/releases)
2. In OpenDeck: Plugins -> Install from file
3. Linux: Download [udev rules](./40-opendeck-mirabox293.rules) and install them by copying into `/etc/udev/rules.d/` and running `sudo udevadm control --reload-rules`
4. Unplug and plug again the device, restart OpenDeck

## Building

### Prerequisites

You'll need:

- A Linux OS of some sort
- Rust 1.87 and up with `x86_64-unknown-linux-gnu` and `x86_64-pc-windows-gnu` targets installed
- Docker
- [just](https://just.systems)

### Building a release package

```sh
$ just package
```

## Acknowledgments

This plugin is based on [opendeck-ss550](https://github.com/MMonkeyKiller/opendeck-ss550) by MMonkeyKiller, which is heavily based on work by contributors of [elgato-streamdeck](https://github.com/streamduck-org/elgato-streamdeck) crate.