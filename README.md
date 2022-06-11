<p align="center"><img src="./src-tauri/icons/128x128.png" align="center" /></p>
<h1 align="center">Random Folder 📁</h1>

<h3 align="center">Random Folder is an <s>lightweight and blazingly fast</s> app that pick a random subfolder inside of a main folder.</h3>

## About

This app was made thanks to [`Tauri`](https://tauri.studio/) which is a **lightweight alternative to Electron** with **rust for the _BackEnd_** and **Any WebFrameworks for the _FrontEnd_**.

I've used [`Preact`](https://preactjs.com/) for the FrontEnd

## Install

### ⚡ `[DISCLAIMER]`:: This app is not production ready and will never be, it's just a side project of mine to test the so called "Tauri" framework

### 🪟 If you have windows you can install this app with the [official installer](https://github.com/Ilingu/random-folder/releases)

To Install this app you will have to bundle it yourself (since it was not design to be installed)

> Before installing this app, be sure to fit the [Tauri's prerequisites](https://tauri.studio/v1/guides/getting-started/prerequisites) and to have [NodeJS](https://nodejs.org) in your machine

1. Clone this repo into you local machine: `git clone https://github.com/Ilingu/random-folder.git`
2. Run `npm install`
3. Run `npx tauri build` It'll build this app into `/src-tauri/target/release/bundle`
4. Choose your installer according to your OS (e.g: msi for windows)
5. Install the app
6. Done!

PS: if you want to **change my code** you can run: `npx tauri dev` to launch the app in _dev mode_

# Purpose and motivation

The idea of this app come from a **very specific** problem that I encounter in a daily basis. So to automate this problem I build this app (logic 🪄)
