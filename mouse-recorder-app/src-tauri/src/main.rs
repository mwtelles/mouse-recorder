#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    sync::Mutex,
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

use once_cell::sync::Lazy;
use rdev::{listen, simulate, Button, Event, EventType};
use tauri::{command, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt};
use tauri_plugin_global_shortcut::plugin::Builder as PluginBuilder;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct MouseAction {
    x: f64,
    y: f64,
    delay_ms: u64,
    action: String,
}

static RECORDING: Lazy<Mutex<Vec<MouseAction>>> = Lazy::new(|| Mutex::new(Vec::new()));
static IS_RECORDING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static IS_PLAYING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static LAST_POS: Lazy<Mutex<(f64, f64)>> = Lazy::new(|| Mutex::new((0.0, 0.0)));

#[command]
fn start_recording() -> Result<(), String> {
    let mut recording = RECORDING.lock().unwrap();
    recording.clear();
    *IS_RECORDING.lock().unwrap() = true;

    spawn(|| {
        let start_time = Instant::now();
        let _ = listen(move |event: Event| {
            if !*IS_RECORDING.lock().unwrap() {
                return;
            }

            let elapsed = start_time.elapsed().as_millis() as u64;

            match event.event_type {
                EventType::MouseMove { x, y } => {
                    *LAST_POS.lock().unwrap() = (x, y);
                    RECORDING.lock().unwrap().push(MouseAction {
                        x,
                        y,
                        delay_ms: elapsed,
                        action: "move".to_string(),
                    });
                }
                EventType::ButtonPress(Button::Left) => {
                    let (x, y) = *LAST_POS.lock().unwrap();
                    RECORDING.lock().unwrap().push(MouseAction {
                        x,
                        y,
                        delay_ms: elapsed,
                        action: "click".to_string(),
                    });
                }
                _ => {}
            }
        });
    });

    Ok(())
}

#[command]
fn stop_recording() -> Result<(), String> {
    *IS_RECORDING.lock().unwrap() = false;
    Ok(())
}

#[command]
fn get_recorded_actions() -> Result<Vec<MouseAction>, String> {
    Ok(RECORDING.lock().unwrap().clone())
}

#[command]
fn stop_playing() -> Result<(), String> {
    *IS_PLAYING.lock().unwrap() = false;
    Ok(())
}

#[command]
fn play_recording(repeat: u32, speed_ms: u64) -> Result<(), String> {
    let actions = RECORDING.lock().unwrap().clone();
    if actions.is_empty() {
        return Err("Nenhuma ação gravada.".into());
    }

    *IS_PLAYING.lock().unwrap() = true;

    spawn(move || {
        for _ in 0..repeat {
            if !*IS_PLAYING.lock().unwrap() {
                break;
            }

            let mut last_time = 0;
            for action in &actions {
                if !*IS_PLAYING.lock().unwrap() {
                    break;
                }

                let wait_time = action.delay_ms.saturating_sub(last_time);
                last_time = action.delay_ms;

                sleep(Duration::from_millis(wait_time.saturating_sub(speed_ms)));

                match action.action.as_str() {
                    "move" => {
                        let _ = simulate(&EventType::MouseMove {
                            x: action.x,
                            y: action.y,
                        });
                    }
                    "click" => {
                        let _ = simulate(&EventType::MouseMove {
                            x: action.x,
                            y: action.y,
                        });
                        let _ = simulate(&EventType::ButtonPress(Button::Left));
                        sleep(Duration::from_millis(50));
                        let _ = simulate(&EventType::ButtonRelease(Button::Left));
                    }
                    _ => {}
                }
            }
        }

        *IS_PLAYING.lock().unwrap() = false;
    });

    Ok(())
}

#[command]
fn click_n_times(x: f64, y: f64, clicks: u32, delay_ms: u64) -> Result<(), String> {
    spawn(move || {
        for _ in 0..clicks {
            simulate(&EventType::MouseMove { x, y }).ok();
            simulate(&EventType::ButtonPress(Button::Left)).ok();
            sleep(Duration::from_millis(30));
            simulate(&EventType::ButtonRelease(Button::Left)).ok();
            sleep(Duration::from_millis(delay_ms));
        }
    });

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(PluginBuilder::new()) // Usa o plugin corretamente
        .setup(|app| {
            let handle = app.handle();

            app.global_shortcut()
                .on_shortcut("F6", move |app, _, _| {
                    let _ = app.emit("toggle-click", ());
                })
                .expect("Erro ao registrar atalho F6");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_recording,
            stop_recording,
            get_recorded_actions,
            play_recording,
            stop_playing,
            click_n_times
        ])
        .run(tauri::generate_context!())
        .expect("erro ao rodar o app");
}
