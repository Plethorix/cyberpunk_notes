#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Color32, FontId, Stroke, Vec2};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// ─────────────────────────────────────────────────────────────────────────────
// Modelo
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Serialize, Deserialize, Default)]
struct Note {
    name: String,
    content: String,
}

#[derive(Serialize, Deserialize, Default)]
struct PersistedData {
    notes: Vec<Note>,
    active: Option<usize>,
}

#[derive(Serialize)]
struct PersistedRef<'a> {
    notes: &'a [Note],
    active: Option<usize>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Estado de guardado
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum SaveStatus {
    Idle,
    Dirty,
    Saved,
    Err,
}

impl SaveStatus {
    fn label(self) -> &'static str {
        match self {
            SaveStatus::Idle => "◦ Sin cambios",
            SaveStatus::Dirty => "● Cambios sin guardar",
            SaveStatus::Saved => "✓ Guardado",
            SaveStatus::Err => "✖ Error al guardar",
        }
    }
    fn color(self) -> Color32 {
        match self {
            SaveStatus::Idle | SaveStatus::Dirty => Color32::from_rgb(170, 170, 170),
            SaveStatus::Saved => Color32::from_rgb(80, 220, 120),
            SaveStatus::Err => Color32::from_rgb(230, 70, 70),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Aplicación
// ─────────────────────────────────────────────────────────────────────────────

struct CyberpunkNotes {
    notes: Vec<Note>,
    active: Option<usize>,
    dirty: bool,
    save_status: SaveStatus,
    new_note_name: String,
    show_delete_modal: bool,
    save_path: PathBuf,
}

impl CyberpunkNotes {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let save_path = resolve_save_path();
        let (notes, active) = match fs::read_to_string(&save_path) {
            Ok(content) => match serde_json::from_str::<PersistedData>(&content) {
                Ok(data) => (data.notes, data.active),
                Err(_) => (Vec::new(), None),
            },
            Err(_) => (Vec::new(), None),
        };

        apply_cyberpunk_visuals(&_cc.egui_ctx);

        Self {
            notes,
            active,
            dirty: false,
            save_status: SaveStatus::Idle,
            new_note_name: String::new(),
            show_delete_modal: false,
            save_path,
        }
    }

    fn mark_dirty(&mut self) {
        self.dirty = true;
        if self.save_status != SaveStatus::Err {
            self.save_status = SaveStatus::Dirty;
        }
    }

    fn save(&mut self) {
        if !self.dirty {
            self.save_status = SaveStatus::Idle;
            return;
        }
        let tmp_path = self.save_path.with_extension("json.tmp");
        let data = PersistedRef {
            notes: &self.notes,
            active: self.active,
        };
        let json = match serde_json::to_string_pretty(&data) {
            Ok(j) => j,
            Err(_) => {
                self.save_status = SaveStatus::Err;
                return;
            }
        };
        if fs::write(&tmp_path, json.as_bytes()).is_err() {
            self.save_status = SaveStatus::Err;
            return;
        }
        if fs::rename(&tmp_path, &self.save_path).is_err() {
            let _ = fs::remove_file(&tmp_path);
            self.save_status = SaveStatus::Err;
            return;
        }
        self.dirty = false;
        self.save_status = SaveStatus::Saved;
    }

    fn create_note(&mut self) {
        let name = if self.new_note_name.trim().is_empty() {
            format!("Nota {}", self.notes.len() + 1)
        } else {
            self.new_note_name.trim().to_string()
        };
        self.notes.push(Note {
            name,
            content: String::new(),
        });
        self.active = Some(self.notes.len() - 1);
        self.new_note_name.clear();
        self.mark_dirty();
    }

    fn delete_active(&mut self) {
        if let Some(i) = self.active.take() {
            if i < self.notes.len() {
                self.notes.remove(i);
                if !self.notes.is_empty() {
                    self.active = Some(if i == 0 { 0 } else { i - 1 });
                }
                self.mark_dirty();
            }
        }
        self.show_delete_modal = false;
    }
}

fn resolve_save_path() -> PathBuf {
    let filename = "cyberpunk_notes.json";
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            return parent.join(filename);
        }
    }
    PathBuf::from(filename)
}

fn apply_cyberpunk_visuals(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    let panel_bg = Color32::from_rgb(0x1a, 0x1a, 0x1a);
    let extreme_bg = Color32::from_rgb(0x0f, 0x0f, 0x0f);
    let faint_bg = Color32::from_rgb(0x22, 0x22, 0x22);
    let text_orange = Color32::from_rgb(0xFF, 0xA5, 0x00);
    let accent = Color32::from_rgb(0xFF, 0x8C, 0x00);
    let sel_bg = Color32::from_rgb(0x55, 0x33, 0x00);

    visuals.override_text_color = Some(text_orange);
    visuals.panel_fill = panel_bg;
    visuals.window_fill = panel_bg;
    visuals.extreme_bg_color = extreme_bg;
    visuals.faint_bg_color = faint_bg;

    visuals.selection.bg_fill = sel_bg;
    visuals.selection.stroke = Stroke::new(1.0_f32, accent);

    let widget_bg = faint_bg;
    visuals.widgets.noninteractive.bg_fill = widget_bg;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, text_orange);
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, accent);

    visuals.widgets.inactive.bg_fill = widget_bg;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, text_orange);
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, accent);

    visuals.widgets.hovered.bg_fill = Color32::from_rgb(0x2a, 0x1a, 0x00);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.5_f32, accent);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.5_f32, accent);

    visuals.widgets.active.bg_fill = Color32::from_rgb(0x33, 0x22, 0x00);
    visuals.widgets.active.fg_stroke = Stroke::new(1.5_f32, accent);
    visuals.widgets.active.bg_stroke = Stroke::new(1.5_f32, accent);

    visuals.widgets.open.bg_fill = widget_bg;
    visuals.widgets.open.fg_stroke = Stroke::new(1.0_f32, text_orange);
    visuals.widgets.open.bg_stroke = Stroke::new(1.0_f32, accent);

    ctx.set_visuals(visuals);

    ctx.all_styles_mut(|style| {
        style.text_styles = [
            (egui::TextStyle::Heading, FontId::monospace(22.0)),
            (egui::TextStyle::Body, FontId::monospace(14.0)),
            (egui::TextStyle::Monospace, FontId::monospace(14.0)),
            (egui::TextStyle::Button, FontId::monospace(14.0)),
            (egui::TextStyle::Small, FontId::monospace(12.0)),
        ]
        .into();
    });
}

impl eframe::App for CyberpunkNotes {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.modifiers.command && i.key_pressed(egui::Key::S)) {
            self.save();
        }

        let close_requested = ctx.input(|i| i.viewport().close_requested());
        if close_requested {
            self.save();
        }

        egui::SidePanel::left("notes_panel")
            .default_width(220.0)
            .resizable(true)
            .width_range(150.0..=400.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("◢ NOTAS ◣");
                });
                ui.add_space(6.0);

                let mut create = false;
                let input_resp = ui.add(
                    egui::TextEdit::singleline(&mut self.new_note_name)
                        .hint_text("Nueva nota...")
                        .desired_width(f32::INFINITY),
                );
                if input_resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    create = true;
                }
                if ui.add_sized([ui.available_width(), 24.0], egui::Button::new("+")).clicked() {
                    create = true;
                }
                if create {
                    self.create_note();
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        for (i, note) in self.notes.iter().enumerate() {
                            let is_active = self.active == Some(i);
                            if ui.selectable_label(is_active, &note.name).clicked() {
                                if self.active != Some(i) {
                                    self.active = Some(i);
                                    self.mark_dirty();
                                }
                            }
                        }
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(idx) = self.active {
                if idx >= self.notes.len() {
                    self.active = None;
                    self.mark_dirty();
                }
            }

            match self.active {
                Some(idx) => {
                    let mut title = self.notes[idx].name.clone();
                    let title_resp = ui.horizontal(|ui| {
                        ui.label("▸ Título:");
                        ui.add(egui::TextEdit::singleline(&mut title).desired_width(f32::INFINITY))
                    });
                    if title_resp.inner.changed() {
                        self.notes[idx].name = title;
                        self.mark_dirty();
                    }

                    ui.horizontal(|ui| {
                        if ui.button("✖ Eliminar").clicked() {
                            self.show_delete_modal = true;
                        }
                    });

                    ui.add_space(6.0);
                    ui.separator();
                    ui.add_space(6.0);

                    let mut content = self.notes[idx].content.clone();
                    let edit_resp = ui.add(
                        egui::TextEdit::multiline(&mut content)
                            .hint_text("Escribe aquí...")
                            .desired_width(f32::INFINITY)
                            .desired_rows(20),
                    );
                    if edit_resp.changed() {
                        self.notes[idx].content = content;
                        self.mark_dirty();
                    }
                }
                None => {
                    let available = ui.available_size();
                    ui.allocate_ui(available, |ui| {
                        ui.with_layout(
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| {
                                ui.heading("◢ Selecciona o crea una nota ◣");
                            },
                        );
                    });
                }
            }
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(self.save_status.color(), self.save_status.label());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let count = self.notes.len();
                    ui.label(format!("{count} nota(s)"));
                    if ui.button("💾 Guardar (Ctrl+S)").clicked() {
                        self.save();
                    }
                });
            });
        });

        if self.show_delete_modal {
            let name = self
                .active
                .and_then(|i| self.notes.get(i))
                .map(|n| n.name.clone())
                .unwrap_or_default();

            egui::Window::new("Confirmar eliminación")
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(format!("¿Eliminar la nota \"{name}\"?"));
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        if ui.button("Sí, eliminar").clicked() {
                            self.delete_active();
                        }
                        if ui.button("Cancelar").clicked() {
                            self.show_delete_modal = false;
                        }
                    });
                });
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Cyberpunk Notes")
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Cyberpunk Notes",
        options,
        Box::new(|cc| Ok(Box::new(CyberpunkNotes::new(cc)))),
    )
}
