use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use enigo::*;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Entry, Orientation};
use std::str::FromStr;
use std::thread;
use std::time::{Duration as StdDuration, Instant};

fn main() {
    // GTK 애플리케이션 초기화
    let app = Application::builder()
        .application_id("com.lidar.weverse_macro")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("시간 입력")
        .default_width(300)
        .default_height(100)
        .build();

    let vbox = gtk4::Box::new(Orientation::Vertical, 5);
    window.set_child(Some(&vbox));

    let date_entry = Entry::new();
    date_entry.set_placeholder_text(Some("YYYY-MM-DD"));
    vbox.append(&date_entry);

    let time_entry = Entry::new();
    time_entry.set_placeholder_text(Some("HH:MM:SS"));
    vbox.append(&time_entry);

    let button = Button::with_label("설정");
    vbox.append(&button);

    button.connect_clicked(move |_| {
        let date_text = date_entry.text().to_string();
        let time_text = time_entry.text().to_string();

        if let (Ok(date), Ok(time)) = (
            NaiveDate::from_str(&date_text),
            NaiveTime::from_str(&time_text),
        ) {
            let mut enigo = Enigo::new();
            let target_datetime = NaiveDateTime::new(date, time);

            println!("목표 시간: {}", target_datetime);

            // 현재 시간 (NaiveDateTime) 계산
            let now = Local::now().naive_local();

            // 목표 시간까지 남은 시간 계산
            let wait_duration = if target_datetime > now {
                target_datetime - now
            } else {
                println!("목표 시간은 이미 지났습니다. 다른 날짜를 설정하세요.");
                return;
            };

            let wait_millis = wait_duration.num_milliseconds();

            if wait_millis > 0 {
                println!("목표 시간까지 대기: {}초", wait_millis as f64 / 1000.0);
                let start = Instant::now();
                let duration_to_sleep = StdDuration::from_millis(wait_millis as u64);
                while start.elapsed() < duration_to_sleep {
                    // CPU 과부하 방지를 위해 1밀리초 대기
                    thread::sleep(StdDuration::from_millis(1));
                }
            }

            // 마우스 좌클릭 수행
            enigo.mouse_click(MouseButton::Left);
            println!("클릭 완료!");
        } else {
            println!("잘못된 날짜 또는 시간 형식입니다.");
        }
    });

    window.show();
}
