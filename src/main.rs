use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use enigo::*;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Entry, Orientation};
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use std::thread;
use std::time::{Duration as StdDuration, Instant};

fn main() {
    let app = Application::builder()
        .application_id("com.lidar.weverse_macro")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn show_popup(window: &ApplicationWindow, message: &str) {
    let dialog = gtk4::MessageDialog::builder()
        .transient_for(window)
        .buttons(gtk4::ButtonsType::Ok)
        .message_type(gtk4::MessageType::Info)
        .text(message)
        .build();

    dialog.connect_response(move |dialog, _| {
        dialog.close();
    });
    dialog.show();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("시간 입력")
        .default_width(300)
        .default_height(170)
        .build();

    let vbox = gtk4::Box::new(Orientation::Vertical, 20);
    vbox.set_margin_top(10);
    vbox.set_margin_bottom(10);
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);
    window.set_child(Some(&vbox));

    let date_entry = Entry::new();
    date_entry.set_placeholder_text(Some("2021-04-01"));
    date_entry.set_margin_bottom(3);
    vbox.append(&date_entry);

    let time_entry = Entry::new();
    time_entry.set_placeholder_text(Some("17:15:07.349"));
    time_entry.set_margin_bottom(3);
    vbox.append(&time_entry);

    let button = Button::with_label("설정");
    button.set_margin_bottom(3);
    vbox.append(&button);

    let current_time_button = Button::with_label("현재 시간 가져오기");
    vbox.append(&current_time_button);

    let date_entry_rc = Rc::new(RefCell::new(date_entry));
    let time_entry_rc = Rc::new(RefCell::new(time_entry));

    let date_entry_clone = date_entry_rc.clone();
    let time_entry_clone = time_entry_rc.clone();
    current_time_button.connect_clicked(move |_| {
        let now = Local::now();

        let date_str = now.format("%Y-%m-%d").to_string();
        let time_str = now.format("%H:%M:%S%.3f").to_string();

        date_entry_clone.borrow().set_text(&date_str);
        time_entry_clone.borrow().set_text(&time_str);
    });

    let window_rc = Rc::new(RefCell::new(window));

    let window_rc_clone = window_rc.clone();
    button.connect_clicked(move |_| {
        let date_entry_clone = date_entry_rc.clone();
        let time_entry_clone = time_entry_rc.clone();

        let window_ref = window_rc_clone.borrow();
        let date_text = date_entry_clone.borrow().text().to_string();
        let time_text = time_entry_clone.borrow().text().to_string();

        if let (Ok(date), Ok(time)) = (
            NaiveDate::from_str(&date_text),
            NaiveTime::from_str(&time_text),
        ) {
            let mut enigo = Enigo::new();
            let target_datetime = NaiveDateTime::new(date, time);

            let now = Local::now().naive_local();

            let wait_duration = if target_datetime > now {
                target_datetime - now
            } else {
                show_popup(
                    &window_ref,
                    "목표 시간은 이미 지났습니다. 다른 날짜를 설정하세요.",
                );
                return;
            };

            let wait_millis = wait_duration.num_milliseconds();

            if wait_millis > 0 {
                // let message = format!("목표 시간까지 대기: {}초", wait_millis as f64 / 1000.0);
                // show_popup(&window_ref, &message);
                let start = Instant::now();
                let duration_to_sleep = StdDuration::from_millis(wait_millis as u64);
                while start.elapsed() < duration_to_sleep {
                    // CPU 과부하 방지를 위해 1밀리초 대기
                    thread::sleep(StdDuration::from_millis(1));
                }

                enigo.mouse_click(MouseButton::Left);
                show_popup(&window_ref, "클릭 완료!");
            }
        } else {
            show_popup(&window_ref, "잘못된 날짜 또는 시간 형식입니다.");
        }
    });

    let window_ref = window_rc.borrow();
    window_ref.show();
}
