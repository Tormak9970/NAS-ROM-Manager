---
title: Folder Structure
editLink: true
---

# Folder Structure

Keeping a consistent folder structure is important, and will help NAS ROM Manager function as intended.

::: tip
I **strongly** recommend following [EmuDeck's](https://emudeck.github.io/cheat-sheet/#cheat-sheets) folder names for systems. It will make your life easier and you won't run into issues with the default parsers.
:::

## ROMs

ROMs are organized into folders by systems. These folders **must match** the folder names listed in the system's parser file. You shouldn't have to worry about this unless you're migrating your collection to NRM. When uploading through the UI, NRM will handle this for you. Depending on the system, ROMs are either stored as a single file or a specific folder structure (see the `Folder Tree Example` below).

## Emulators

Simple and straightforward, emulators are stored as a single file, usually a `.zip` or an executable.


## BIOS Files

Bios files are organized into folders by systems. These folders **must match** the folder names listed in the system's parser file. You shouldn't have to worry about this unless you're migrating your collection to NRM. When uploading through the UI, NRM will handle this for you.


## DLCs / Updates

DLC and Update files are organized into folders by systems and ROM ID. These folders **must match** the folder names listed in the system's parser file. You shouldn't have to worry about this unless you're migrating your collection to NRM. When uploading through the UI, NRM will handle this for you.


## Folder Tree Example

```
library
├── roms
│   ├── switch
│   │   ├── Game 2 - Return of the Game.nsp
│   │   └── another_game.xci
│   ├── n3ds
│   │   ├── Game 2 - Return of the Game.3ds
│   │   └── another_game.3ds
│   └── ps3
│       └── Very Cool Game
|           └── PS3_GAME
|               └── USRDIR
|                   └── eboot.bin
├── emulators
|   ├── ryujinx-1.2.3.exe
|   ├── citra-2.2.2.zip
|   └── cemu-1.1.1.zip
├── bios
|   ├── ps1
|   |   └── firmware-3.2.6.zip
|   └── switch
|       ├── prod-keys-1.2.3.zip
|       └── firmware-1.2.3.zip
├── updates
|   ├── switch
|   |   ├── 0ao5tki0294rc3
|   |   |   └── ssbu-update-1.2.3.nsp
|   |   └── 0b6hhyl0kj5uqh
|   |       └── metroid-prime-update-1.2.3.xci
|   └── ps1
└── dlc
    ├── gba
    |   └── 0nkygqv18kx10m
    |       └── zelda-oracle-of-ages-update-1.3.4.gba
    └── nes
```