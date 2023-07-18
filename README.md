<h1 align="center">Analyze screen by Vector Scope with Tauri (Rust + JS)</h1>

![example](/Sample_1.jpg)
![example](/Sample_2.jpg)
![example](/Sample_3.jpg)

<div align="center">
Lightweight screen color analyze tool powered by Tauri.
</div>

<h1>Features</h1>

### Basic feature

- Show vector scope from screen content
- Vector scope window is always on top; not blocked by other windows
- Vector scope window is not captured; the analyze result is not containing vector scope itself
- Stay on system tray, easy to access all feature
- Efficient standby
  - Standby RAM usage: ~200MB
  - Standby CPU usage: almost 0%
    - Checked on my MacBook (M1 Pro)

### Refresh view

- Auto refresh: 1 sec interval
- Manual refresh by: `Command Or Control + Shift + R` (from anywhere. not require focus on window.)
  - This also stops auto refresh for less CPU usage

### Area speficied capture

- Selected area only analyze
- Easy to set or reset area

<h1>TODO</h1>

- [x] Add screen shot capability
- [x] Add vector scope
  - [x] Manual refresh feature
  - [x] Capturing area specified analyze
  - [ ] Better UI
- [x] Add waveform
  - [ ] Add luminance view
  - [ ] Add separate RGB view
- [ ] Add around mouse local analyze feature
- [ ] Add user configuration
  - [ ] Analyze resolution
  - [ ] Global shortcut
  - [ ] Auto refresh delay
- [x] Fix memory leak (Caused by Webkit?)
  - Use Object URL for less leak
  - And workaround: reopen window
- [ ] Create app icon
- [ ] Cleanup code
- And more...

<h2>Thanks to the developers for the wonderful libraries.</h1>
