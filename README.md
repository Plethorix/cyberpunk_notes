# Cyberpunk Notes

> A minimal, terminal-aesthetic notes manager for the desktop.  
> Dark. Orange. Monospaced. Zero disk waste.
>
> ## What is this?

**Cyberpunk Notes** is a lightweight, native desktop application for managing
multiple notes at once, wrapped in a cyberpunk/terminal aesthetic — dark
backgrounds, warm orange text (`#FFA500`), orange accents (`#FF8C00`), and
monospaced fonts throughout.

It's built in **Rust** with [`eframe`/`egui`](https://github.com/emilk/egui),
compiles to a **single binary of 3.8 MB**, and makes **zero disk writes during
a session** — your notes live entirely in RAM and are persisted **once**, when
you close the app.

No accounts. No cloud. No telemetry. No background processes. No autosave
churning your SSD.

---

## Features

- 📝 **Multiple simultaneous notes** — create, rename, edit, delete
- 🗂️ **Sidebar** with the full list, click to switch
- ✍️ **Large central editor** for the active note
- ⚠️ **Confirmation modal** before deleting anything
- 💾 **Persist on close only** — no disk writes while you type
- 🧠 **Restores state** — reopening the app brings back every note, its
  content, and which one was active
- ⚡ **Ctrl+S** for a manual save (also available as a button)
- 🎨 **Cyberpunk theme** — `#1a1a1a` background, `#FFA500` text, monospace everywhere
- 🪟 **Native window** — 900×600 default, resizable down to 400×300
- 🚫 **No console window** on Windows (release builds)
- 📦 **Tiny footprint** — ~3–6 MB executable, minimal memory
