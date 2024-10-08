use std::{thread, time::Duration};
use fltk_theme::{widget_themes, WidgetTheme, ThemeType};
use enigo::*;
use fltk::{app, button::Button, frame::Frame, input::{self, Input}, prelude::*, window::Window};
#[allow(unused_mut)]
fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 130, "RustyTyper");
    let widget_theme = WidgetTheme::new(ThemeType::HighContrast);
    widget_theme.apply();

    let mut inp = Input::new(50, 0, 130, 30, "Input");
    let mut dlt = Button::new(270, 100, 65, 30, "-");
    let mut delay = Input::new(50, 40, 130, 30, "Delay");
    let mut add = Button::new(270, 0, 65, 30, "+");
    let mut times = Input::new(360, 0, 65, 30, "X?");
    
    let mut outp = Frame::new(50, 40, 400, 30, "Output:");
    let mut gen = Button::new(0, 80, 400, 30, "Generate");

    let toplace = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let toplace_add = toplace.clone();
    let toplace_gen = toplace.clone();

    let mut outp_add = outp.clone();
    add.set_callback(move |_| {
        if let Ok(repeat) = times.value().parse::<usize>() {
            if !inp.value().is_empty() {
                let mut toplace = toplace_add.borrow_mut();
                for _ in 0..repeat {
                    toplace.push(inp.value().clone());
                }
                outp_add.set_label(&toplace.join(", "));
            }
        }
    });

    gen.set_callback(move |_| {
        thread::sleep(Duration::from_secs(3));
        let delay_ms = delay.value().parse::<u64>().unwrap_or(100); // Default to 100 ms
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        for text in toplace_gen.borrow().iter() {
            enigo.text(text);
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
        }
    });

    let mut outp_dlt = outp.clone();
    dlt.set_callback(move |_| {
        let mut toplace = toplace.borrow_mut();
        toplace.clear();
        outp_dlt.set_label("");
    });
    wind.end();
    wind.show();
    app.run().unwrap();
}