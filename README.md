# Bevy Template

An opinionated [Bevy] project template with a configuration script to run for the more complex parts (like setting up the fast-compile setup).

## Quick Start

- Click on [Use this template] and choose your new project name
- Clone your new repository to your development location (local laptop, etc.)
- Run `./setup` (macOS, Linux) to complete the setup

## Features:
- **Template-based.** - No Git history comes along for the ride. You can create private repositories directly from the template.
- **Git configured** - `.gitignore` is already set up for you.
- **Not license-restricted** - The included license is _for the template itself_.  Projects starting with this template may use any license (or no license at all).
- **Startup script** - Run the interactive setup script to configure the project beyond the template.  Configuration choices include:
  - Set the package name
  - Set the package author
  - Update to the latest version of Bevy
  - Set up the fast-compile config file
  - Install `LLD` on Linux (not working, yet - I need a Linux collaborator to submit a PR!)
  - Install `zld` on macOS
  - Install the nightly compiler
  - Configure the project to use the nightly compiler
  - Replace the README.md file
  - Delete the LICENSE file

[Bevy]: https://github.com/bevyengine/bevy
[Use this template]: https://github.com/CleanCut/bevy_template/generate

:sparkling_heart: Help Needed!
We need help from Linux and Windows users!  For example:
- The `setup` script _ought_ to mostly work on Linux, but it needs to be tested
- `setup` needs code to install LLD on various Linux Distros
- The `setup` script doesn't support Windows at all, yet! We need to figure out a way to do the equivalent stuff for Windows users.
