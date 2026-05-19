# Racing Wheel Sim Configurator 🏎️🐧

A modern, lightweight, and hardware-agnostic graphical interface designed to configure racing wheels natively on Linux. This project aims to fill the gap in the Linux gaming ecosystem by providing an intuitive control panel—similar to Logitech G Hub—but tailored strictly for simulation hardware.

## ✨ Key Features (Roadmap)
* **Zero Installation:** Compiles into a single, ultra-lightweight portable binary (thanks to Rust & Slint). No heavy daemon or background frameworks required.
* **Hardware Agnostic:** Architected using a clean Hardware Abstraction Layer (HAL) to support multiple brands via generic interfaces (`Traits`).
* **Logitech G923 Support:** Initial focus on exposing full hardware capabilities (rotation range, centering force, force feedback tuning) through `sysfs`.
* **Game Profiles (Planned):** Automatic profile switching based on Steam AppIDs or active Linux processes.

## 🛠️ Built With
* **Language:** Rust 🦀 (For safety, system-level performance, and raw speed)
* **UI Framework:** Slint 🎨 (For a fluid, modern, GPU-rendered native GUI)
* **Platform:** Linux Native (Optimized for performance on lightweight environments like Tiling Window Managers / TileOS)

## 📐 Architecture Overview
The codebase is strictly separated into three independent layers to ensure scalability when adding new manufacturers:

1. **Frontend (UI):** Declarative Slint files handling user input and visuals.
2. **Core Orchestrator:** Rust logic managing JSON game profiles and system process scanning.
3. **Hardware Abstraction Layer (HAL):** Generic Rust Traits that translate software sliders into vendor-specific `sysfs` or `udev` writes.

## 🤝 Contributing & Hardware Support
If you are a hardware manufacturer (Logitech, Thrustmaster, Fanatec, Moza, etc.) or a community developer interested in adding native support for a specific wheel model, please check our HAL interface structure. We are open to implementing new modules!