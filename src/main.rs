#![windows_subsystem = "windows"]

extern crate chrono;

mod toast;
mod tray;

// 윈도 Toast의 App ID
const APP_ID : &str = "Remember 0416";

// Toast 메시지 제목
const TITLE : &str = "2014.4.16.  SEWOL.";

// 나무위키 사고정보 페이지
const URL : &str = "https://namu.wiki/w/%EC%B2%AD%ED%95%B4%EC%A7%84%ED%95%B4%EC%9A%B4%20%EC%84%B8%EC%9B%94%ED%98%B8%20%EC%B9%A8%EB%AA%B0%20%EC%82%AC%EA%B3%A0";

// 프로그램 정보 페이지
const PROGRAM_INFO : &str = "https://github.com/Regentag/Remember0416";

fn main() -> windows::Result<()> {
    use std::time::Duration;
    use bindings::{
        Windows::Win32::WindowsAndMessaging::{
            self,
            HWND, MSG,
            WM_QUIT,
            PEEK_MESSAGE_REMOVE_TYPE }
    };

    // Create tray icon
    let (_tray, tray_rx) = tray::new();
    let d = Duration::from_nanos(1);

    // windows message
    let mut msg = MSG::default();
    let h_wnd = HWND::default();

    'event: loop{
        // see: https://blog.naver.com/valdo4472/120045875471
        unsafe {
            while WindowsAndMessaging::PeekMessageA(&mut msg, h_wnd, 0, 0, PEEK_MESSAGE_REMOVE_TYPE::PM_REMOVE).as_bool() {
                if msg.message == WM_QUIT {
                    break 'event;
                } else {
                    WindowsAndMessaging::TranslateMessage(&msg);
                    WindowsAndMessaging::DispatchMessageA(&msg);
                }
            }

            // handle tray icon event
            let tray_msg = tray_rx.recv_timeout(d);
            if let Ok(ev) = tray_msg {
                if handle_trayicon_event(ev) {
                    break 'event;
                }
            } else {
                WindowsAndMessaging::WaitMessage();
            }
        }
    }

    return Ok(());
}

fn handle_trayicon_event( event: tray::Events ) -> bool {
    match event {
        tray::Events::Click => {
            show_toast();
        },
        tray::Events::ShowOnlineInfo => {
            open_browser(URL);
        }
        tray::Events::About => {
            open_browser(PROGRAM_INFO);
        }
        tray::Events::Exit => {
            return true;
        },
    }

    false
}

// 메시지 출력
fn show_toast() {
    let msg = format!(
        "우리가 기억하는 한\n그들은 절대 침몰하지 않습니다.\n\n사고일로부터 {}일.",
        day_after()
    );
    let icon = get_resource_path("ribbon.ico");

    let mut noti = toast::Notification::new();
    noti.app_id = APP_ID;
    noti.title = TITLE;
    noti.message = &msg[..];
    noti.icon = &icon[..];
    noti.duration = toast::ToastDuration::Long;
    noti.actions.push(toast::ToastAction::new("system", "닫기", "dismiss"));
    noti.push().ok();
}

// URL을 브라우저에서 열기. (Windows Only.)
fn open_browser(url: &str) {
    use std::process::Command;

    Command::new("rundll32")
        .args(&["url.dll,FileProtocolHandler", url])
        .spawn().ok();
}

// 사고일로부터의 경과일수를 계산하여 반환.
// 2014. 4. 16. 00:00을 기준으로 합니다. (나무위키의 일자 계산과 일치)
fn day_after() -> i64 {
    use chrono::prelude::*;

    let now = Local::today();
    let then = Local.ymd(2014, 4, 16);

    let diff = now.signed_duration_since(then);

    diff.num_days()
}

// 실행파일이 설치된 폴더를 기준으로 리소스 파일의 전체경로 반환
fn get_resource_path(file_name: &str) -> String {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.as_path().parent() {
            let res = dir.join(file_name);

            if let Some(res_path) = res.as_path().to_str() {
                return String::from(res_path);
            }
        }
    }

    String::from(file_name)
}
