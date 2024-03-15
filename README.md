# Amie for Windows (unofficial)
This repository contains the source code of my personal attempt to port Amie over to Windows. I used tauri to basically embed the website into an application with some extra features.

## Installation
You can download the latest release from the releases page. The executable is the installer and will do everything for you.

## Additional Features
Compared to the web app, this version has the following additional features:
- **System Tray Icon**: The app can be minimized to the system tray. It will continue to run in the background and can be restored by clicking on the tray icon or right-clicking on it and selecting 'Open Amie'.
- **Current Day Display**: The current day is displayed in the System Tray Icon, just like the favicon on the website.

## To-Do
- [ ] Fix date display when the date changes (e.g. at midnight)

## Not coming
- **Notifications**: Due to the complexity of the notification system, it is way too difficult to implement.

## ❤️ Credits
- [Amie](https://amie.so) for the literal app. I just made it run on Windows.
- [Tauri](https://tauri.studio) for making it easy and lightweight to build.
- vonPB for the whole idea and recommendation to use Tauri.