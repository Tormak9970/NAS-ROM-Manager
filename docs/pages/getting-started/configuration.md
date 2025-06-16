---
title: Configuration
editLink: true
---

# Configuration

There's a variety of Environment variables and Volumes you can use to to configure NRM.

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

