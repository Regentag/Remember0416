use trayicon::*;
use std::sync::mpsc::{ Receiver };

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Events {
	Click,
	ShowOnlineInfo,
	About,
	Exit,
}

/// 트레이 아이콘 생성.
pub fn new() -> (TrayIcon<Events>, Receiver<Events>) {
	let (s, r) = std::sync::mpsc::channel::<Events>();
	let icon_bytes = include_bytes!("ribbon.ico");

	let tray_icon = TrayIconBuilder::new()
		.sender(s)
		.icon_from_buffer(icon_bytes)
		.tooltip("Remember 0416")
		.on_click(Events::Click)
		// .on_double_click(Events::DoubleClick)
		.menu(
			MenuBuilder::new()
				.item("세월호 침몰 사고 정보(나무위키)", Events::ShowOnlineInfo)
				.item("프로그램 정보", Events::About)
				.separator()
				.item("종료", Events::Exit)
		)
		.build()
		.unwrap();

	(tray_icon, r)
}
