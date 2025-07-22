use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use enigo::{Direction::Click, Enigo, Mouse, Settings};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Entry, Orientation};
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use std::time::Duration as StdDuration;

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

    let enigo = Rc::new(RefCell::new(Enigo::new(&Settings::default()).ok()));

    if enigo.borrow().is_none() {
        button.set_sensitive(false);
        button.set_tooltip_text(Some(
            "입력 제어 초기화에 실패했습니다. Wayland 권한을 확인하세요.",
        ));
        show_popup(
            &window,
            "입력 제어(Enigo) 초기화에 실패했습니다.\nWayland 환경에서는 원격 데스크톱 제어 권한이 필요할 수 있습니다.",
        );
    }

    let date_entry_rc = Rc::new(RefCell::new(date_entry));
    let time_entry_rc = Rc::new(RefCell::new(time_entry));

    let timer_active = Rc::new(RefCell::new(false));

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
    let timer_active_clone = timer_active.clone();
    let enigo_clone = enigo.clone();
    button.connect_clicked(move |_| {
        let date_entry_clone = date_entry_rc.clone();
        let time_entry_clone = time_entry_rc.clone();
        let timer_active_clone = timer_active_clone.clone();
        let enigo_clone = enigo_clone.clone();

        let window_ref = window_rc_clone.borrow();
        let date_text = date_entry_clone.borrow().text().to_string();
        let time_text = time_entry_clone.borrow().text().to_string();

        if let (Ok(date), Ok(time)) = (
            NaiveDate::from_str(&date_text),
            NaiveTime::from_str(&time_text),
        ) {
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
                if *timer_active_clone.borrow() {
                    show_popup(&window_ref, "이미 타이머가 설정되어 있습니다.");
                    return;
                }

                *timer_active_clone.borrow_mut() = true;

                let window_weak = window_ref.downgrade();
                let timer_active_for_callback = timer_active_clone.clone();

                glib::timeout_add_local(StdDuration::from_millis(wait_millis as u64), move || {
                    *timer_active_for_callback.borrow_mut() = false;

                    if let Some(enigo) = enigo_clone.borrow_mut().as_mut() {
                        let _ = enigo.button(enigo::Button::Left, Click);
                        if let Some(window) = window_weak.upgrade() {
                            show_popup(&window, "클릭 완료!");
                        }
                    } else {
                        if let Some(window) = window_weak.upgrade() {
                            show_popup(&window, "오류: Enigo 인스턴스를 사용할 수 없습니다.");
                        }
                    }

                    glib::ControlFlow::Break
                });

                let wait_seconds = wait_millis as f64 / 1000.0;
                let message = format!("타이머 설정됨: {:.3}초 후 클릭", wait_seconds);
                show_popup(&window_ref, &message);
            }
        }
    });

    let window_ref = window_rc.borrow();
    window_ref.show();
}
