use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use enigo::*;
use std::thread;
use std::time::{Duration as StdDuration, Instant};

fn main() {
    let mut enigo = Enigo::new();

    // 목표 날짜와 시간 설정
    let target_date = NaiveDate::from_ymd_opt(2024, 4, 4).unwrap();
    let target_time = NaiveTime::from_hms_milli_opt(21, 30, 10, 234).unwrap();
    let target_datetime = NaiveDateTime::new(target_date, target_time);

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
}
