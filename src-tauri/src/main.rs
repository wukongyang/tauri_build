#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use window_shadows::set_shadow;
use tauri::{
	Runtime,Manager,AboutMetadata, CustomMenuItem, Menu, MenuEntry, MenuItem, Submenu, WindowBuilder,
	WindowUrl,
};



fn main() {
	let ctx = tauri::generate_context!();

	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![])
		.menu(Menu::with_items([
			#[cfg(target_os = "macos")]
			MenuEntry::Submenu(Submenu::new(
				&ctx.package_info().name,
				Menu::with_items([
					MenuItem::About(ctx.package_info().name.clone(), AboutMetadata::default())
						.into(),
					MenuItem::Separator.into(),
					// MenuItem::Services.into(),
					MenuItem::Separator.into(),
					MenuItem::Hide.into(),
					MenuItem::HideOthers.into(),
					MenuItem::ShowAll.into(),
					MenuItem::Separator.into(),
					MenuItem::Quit.into(),
				]),
			)),
			// MenuEntry::Submenu(Submenu::new(
			// 	"操作",
			// 	Menu::with_items([MenuItem::CloseWindow.into()]),
			// )),
			MenuEntry::Submenu(Submenu::new(
				"操作",
				Menu::with_items([
					// MenuItem::Undo.into(),
					// MenuItem::Redo.into(),
					// MenuItem::Separator.into(),
					MenuItem::Cut.into(),
					MenuItem::Copy.into(),
					MenuItem::Paste.into(),
					// #[cfg(not(target_os = "macos"))]
					// MenuItem::Separator.into(),
					// MenuItem::SelectAll.into(),
				]),
			)),
			// MenuEntry::Submenu(Submenu::new(
			// 	"View",
			// 	Menu::with_items([MenuItem::EnterFullScreen.into()]),
			// )),
			// MenuEntry::Submenu(Submenu::new(
			// 	"Window",
			// 	Menu::with_items([MenuItem::Minimize.into(), MenuItem::Zoom.into()]),
			// )),
			// // You should always have a Help menu on macOS because it will automatically
			// // show a menu search field
			// MenuEntry::Submenu(Submenu::new(
			// 	"Help",
			// 	Menu::with_items([CustomMenuItem::new("Learn More", "Learn More").into()]),
			// )),
		]))
		.on_menu_event(|event| {
			// let event_name = event.menu_item_id();
			// match event_name {
			// 	"Learn More" => {
			// 		let url =
			// 			"https://github.com/probablykasper/tauri-sveltekit-template".to_string();
			// 		shell::open(&event.window().shell_scope(), url, None).unwrap();
			// 	}
			// 	_ => {}
			// }
		})
		.setup(|app| {
			let splashscreen_window = app.get_window("splashscreen").unwrap();
			let main_window = app.get_window("main").unwrap();
			// we perform the initialization code on a new task so the app doesn't freeze
			tauri::async_runtime::spawn(async move {
			  println!("Initializing...");
			  std::thread::sleep(std::time::Duration::from_secs(3));
			  println!("Done initializing.");
			  splashscreen_window.close().unwrap();
			  main_window.show().unwrap();
			  set_shadow(&main_window, true).expect("Unsupported platform!");
			});
			Ok(())
		  })
		.run(ctx)
		.expect("error while running tauri application");
}