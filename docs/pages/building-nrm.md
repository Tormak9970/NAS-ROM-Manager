---
title: Building NRM
editLink: true
---

# Building NAS ROM Manager

## Setting Up the Enviroment
Before you get started, you will need to install the following:

 - [Node.js](https://nodejs.org/en/) - The frontend runtime.
 - [Bun](https://bun.sh/) - The package manager used for the frontend.
 - [Rust](https://www.rust-lang.org/) - The language the backend is written in.

## Cloning the Program
The next step is to get a local copy of the repository. This can be done many ways, I recommend forking this repository and cloning that. <br/>

## Installing Dependencies

:::tip
I recommend [VSCode](https://code.visualstudio.com/) for the best experience working on Nas ROM Manager.
:::

Once you have cloned the repository and opened it in your preffered Editor/IDE, you will need to install the program's dependencies. To do this, you will need to run the following commands: <br/>

```
cd ./web
bun install
cargo install
```

## Running the Application
Now you are finally ready to get the app up and running! Assuming everything is set up correctly, run the following, each in their own terminal:<br/>

```
cd ./web
bun run "dev:svelte"
```
```
cd ./web
bun run "dev:rust"
```

## Building With Your Changes
Once you have made your edits and are ready to containerize, you can build NRM with [docker](https://www.docker.com/products/docker-desktop/):
```
cd ./docker
docker compose -f docker-compose.yml up
```

:::warning
Make sure you've added all of the needed environment variables to `docker-compose.yml`.
:::