#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use turn_order::turn_order::{TurnOrder, creature::status_effect};
use std::{io, str};

fn main() -> eframe::Result {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 480.0]),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut initiative_input = "3".to_owned();

    let mut order = TurnOrder::new();

    /*
    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    })

    */

    eframe::run_simple_native("Initiative Tracker", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let name_label = ui.label("Creature Name:");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("Creature Initiative:");
                ui.text_edit_singleline(&mut initiative_input)
                    .labelled_by(name_label.id);
            });
            if ui.button("Add Creature").clicked() {
                let initiative: isize;

                match try_get_type::<isize>(&initiative_input) {
                    UserInputResult::Ok(num) => order.add_creature(name.to_string(), num),
                    UserInputResult::Err(e) => (),
                    UserInputResult::None => ()

                }
            }
            for creature in order.creatures() {
                ui.label(format!("{creature}"));
            }
        });
    })
}

fn try_get_type<T>(user_input: &str) -> UserInputResult<T, String> 
where
    T: str::FromStr,
    <T as str::FromStr>::Err: ToString
{

    if user_input.is_empty() {
        return UserInputResult::None;
    }

    match user_input.trim().parse::<T>() {
        Ok(user_num) => UserInputResult::Ok(user_num),
        Err(e) => UserInputResult::Err(e.to_string())
    }
}

enum UserInputResult<T, E> {
    Ok(T),
    Err(E),
    None
}

