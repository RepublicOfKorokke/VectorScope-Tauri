<h1 align="center">Analyze screen by Vector Scope with Tauri</h1>

![example](/Sample_1.jpg)
![example](/Sample_2.jpg)
![example](/Sample_3.jpg)

## What's this?

Lightweight screen color analyze tool powered by Tauri.

I want a simple yet low CPU / RAM usage app, so I made it.

## TODO List

- [x] Add screen shot capability
- [x] Add vector scope
  - [x] Manual refresh feature
  - [ ] Capturing area specified analyze
  - [ ] Better UI
- [ ] Add waveform
- [ ] Add user configuration
  - [ ] Analyze resolution
  - [ ] Global shortcut
  - [ ] Auto refresh delay
- [x] Fix memory leak (Caused by Webkit?)
  - Use Object URL for less leak
  - And workaround: reopen window
- [ ] Create app icon
