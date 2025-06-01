<h1 align="center">
  <a name="logo" href=""><img src="./web/static/github-banner.png" /></a>
</h1>

<p align="center">
  <a href="https://github.com/Tormak9970/NAS-ROM-Manager/releases"><img src="https://img.shields.io/github/v/release/Tormak9970/NAS-ROM-Manager?label=version&style=flat-square" /></a>
  <img src="https://img.shields.io/docker/pulls/Tormak9970/NAS-ROM-Manager?logo=Docker&style=flat-square" />
  <a href="https://www.gnu.org/licenses/gpl-3.0.en.html"><img src="https://img.shields.io/github/license/Tormak9970/NAS-ROM-Manager?style=flat-square" /></a>
  <!-- <a href="https://crowdin.com/project/NAS-ROM-Manager"><img src="https://img.shields.io/badge/-translate-brightgreen?style=flat-square&logo=Crowdin" /></a> -->
  <br>
  <br>
</p>

# NAS ROM Manager

NAS ROM Manager is a lightweight, feature rich management app for your retro game collection. Its easy to setup and use, and looks good to boot.

# Configuration

## Environment Variables

| Variable                  | Required | Description |
| :-----------------------: | :------: | :--------- |
|  `NRM_USERNAME`           |   Yes    | Your username for authentication |
|  `NRM_PASSWORD`           |   Yes    | Your password for authentication |
|  `SGDB_API_KEY`           |   Yes    | Your SteamGridDB API Key |
|  `IGDB_CLIENT_ID`         |   Yes    | Your IGDB Client ID |
|  `IGDB_CLIENT_SECRET`     |   Yes    | Your IGDB Client Secret |
|  `UPLOAD_CLEAN_SCHEDULE`  |   No     | The interval at which NRM cleans incomplete uploads |
|  `NRM_VERSION`            |   No     | The App Version |
|  `BUILD_DATE`             |   Yes    | The date the app was built |

## Volumes

|  Volume  |    Mount Point    | Description             |
|:-------: | :---------------: | :--------------------- |
|  Config  |   `/config/NRM`   | This is the folder where all of NRM's config files will be stored. Not mapping a volume to it means your settings will be lost on container restart. |
| Library  |   `/library`      | This is the folder containing all of your ROMs, Emulators, and BIOS files. |


# Installation

## Docker

 - TODO


# Features
 - Blazing fast load times
 - Rich metadata support
 - Large set of recognized systems, with the ability to add your own
 - Customizeable banner and cover Art for each ROM
 - Mobile friendly design
 - Clean / themeable Interface
 - Easy, secure authentication
 - Builtin upload / download Systems, you never have to access your ROMs folder again

<!-- # Translations
If you're native language (or a language you speak) is not currently supported, please consider contributing to NRM's translations! You can help by heading to [https://crowdin.com/project/svunes](https://crowdin.com/project/tunistic) and submitting translations. If a language is not listed there, please submit a GitHub issue and I will add it asap. -->


# Building NAS ROM Manager
> **Please note:** you may edit and distrubute this program as you see fit but you must retain the license and the copyright notice I included (feel free to mark your contributions as I have). <br/>

### Setting Up the Enviroment
Before you get started, you will need to install the following:

 - [Node.js](https://nodejs.org/en/) - The frontend runtime.
 - [Bun](https://bun.sh/) - The package manager used for the frontend.
 - [Rust](https://www.rust-lang.org/) - The language the backend is written in.

### Cloning the Program
The next step is to get a local copy of the repository. This can be done many ways, I recommend forking this repository and cloning that. <br/>

### Installing Dependencies
Once you have cloned the repository and opened it in your preffered Editor/IDE (I recommend [VSCode](https://code.visualstudio.com/)), you will need to install the program's dependencies. To do this, you will need to run two sets of commands: <br/>
First:<br/>
```
cd ./web
bun install
```
Next:<br/>
```
cd ../server
cargo install
```

### Running the Application
Now you are finally ready to get the app up and running! Assuming everything is set up correctly, run the following, each in their own terminal:<br/>
```
cd ./web
bun run "dev:svelte"
```
```
cd ./web
bun run "dev:rust"
```

### Building With Your Changes
Once you have made your edits and are ready to containerize, you can build NRM with [docker](https://www.docker.com/products/docker-desktop/):
```
cd ./docker
docker compose -f docker-compose.yml up
```
**IMPORTANT:** Make sure you've added all of the needed environment variables to `docker-compose.yml`.

# Acknowledgements
APIs Used:

 - [IGDB](https://www.igdb.com/) - Used for fetching ROM metadata.
 - [SGDB](https://steamgriddb.com/) - Used to get cover / banner art for ROMs and Systems.

References:

 - [Svelte Md3](https://ktibow.github.io/m3-svelte/) - Many of the base components are heavily modified version of components from Svelte Md3.

Libraries:

 - [Material Icons](https://fonts.google.com/icons) - All of the app's icons came from here.

# License
 - This program is licensed under the [GNU General Public License Version 3](https://www.gnu.org/licenses/#GPL)
 - Please provide appropriate credit for code usage

Copyright Travis Lane