<div align="center">

# <img src="./static/logo-name.svg" alt="swiping" width=200/>

![Typescript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)
![Svelte](https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00)
![RUST](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

[![Publish](https://github.com/cgund98/fotobinder/actions/workflows/publish.yaml/badge.svg)](https://github.com/cgund98/fotobinder/actions/workflows/publish.yaml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

_Fotobinder is a free desktop app that helps you organize, tag, and search your vast library of photos. Available for Windows, Linux, and MacOS\*_

[Features](#features) •
[Installation](#installation) •
[Contributing](#contributing)

<video src="https://github.com/cgund98/fotobinder/assets/17605568/9ef61079-370f-4137-a461-b556543a2908" alt="swiping" height="600"/>

</div>

## Features

- :camera: **Source images** from anywhere on your local file system.
- :label: Create **hierarchical tags** and assign them to individual images or entire folders.
- :mag: Build **modular queries** to search for specific tag combinations.
- :file_folder: Save images for later by adding them to **collections**.

### Donations

<a href="https://www.buymeacoffee.com/cgund98" target="_blank" title="buymeacoffee" align="right">
  <img src="https://iili.io/JIYMmUN.gif"  alt="buymeacoffee-blue-badge" style="width: 120px;" align="right">
</a>

I made this project simply to improve my own efficiency. I don't intend on ever charging anyone a dime to use it.

If you happen to find it worthwhile and want to say thanks, feel free to [leave me a tip](https://www.buymeacoffee.com/cgund98)!

## Installation

To install Fotobinder, simply visit the [Releases Page](https://github.com/cgund98/fotobinder/releases) and download the installer for your OS of choice.

### Available Installers

1. Windows (`.exe`, `.msi`)
2. Linux (`.AppImage`, `.deb`)
3. \*MacOS - _Unfortunately you'll have to build this yourself. This can be done by following the [developer](#development-environment) instructions._

### Report a Bug

If you encounter a bug, please reach out directly or create an [issue](https://github.com/cgund98/fotobinder/issues).

## Contributing

Is the app missing something important? Feel free to either create an [issue](https://github.com/cgund98/fotobinder/issues) or build the feature yourself!

The application is entirely open-source and any contributions are welcome.

### Development Environment

The app was built using the Rust framework [Tauri](https://tauri.app). Please begin by installing the dependencies specified in Tauri's [Prerequisites Page](https://tauri.app/v1/guides/getting-started/prerequisites/).

After installing Rust, you'll need to install [NodeJS](https://nodejs.org/en/download).

Then you can simply build the application as follows:

```bash
# Install JavaScript dependencies
npm install

# Start the application in development mode
npm run tauri dev

# OR create a production build
npm run tauri build
```

### Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

### Cutting a new release

Releases are created entirely via GitHub Actions. The process is as follows:

1. Create a [Pull Request](https://github.com/cgund98/fotobinder/pulls).
2. Request review of PR created in step 1.
3. Once the PR is merged into main, a release will be created with the version specified in `src-tauri/tauri.conf.json`.
