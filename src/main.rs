use enigo::*;
use fltk::{app, button::Button, frame::Frame, input::{self, Input}, prelude::*, window::Window};

fn main() {
    let mut toplace = vec![];
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "RustyTyper");

    let mut inp = Input::new(0, 0, 130, 150, "input");
    let mut delay = Input::new(131, 0, 130, 150, "delay");
    let mut add = Button::new(261, 0, 65, 150, "+");
    let mut times = Input::new(326, 0, 65, 150, "X?");

    let mut outp = Frame::new(0, 151, 260, 150, "output");
    let mut gen = Button::new(261, 151, 130, 150, "gen");

    add.set_callback(move |_| {
        if inp.value().len() > 0 {
            for i in times.value().parse<i32>().unwrap() {
                toplace.push(inp.value());
            }
        }
    });
    gen.set_callback(move |_| {
        outp.set_label(toplace.join("").as_str());
        let mut stn = enigo::Enigo::new(&Settings::default()).unwrap();
        for i in &toplace {
            stn.text(i);
            std::thread::sleep(std::time::Duration::from_secs(delay.value().parse::<u64>().unwrap()));
        }
    });

    wind.end();
    wind.show();
    app.run();
}