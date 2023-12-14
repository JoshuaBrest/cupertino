use cupertino_sys::{
    core::CGRect,
    appkit::{NSApplication, NSWindow, NSWindowStyleMask, NSApplicationActivationPolicy, NSMenu, NSMenuItem},
};

fn main() {
    let app = NSApplication::new();
    app.set_activation_policy(NSApplicationActivationPolicy::Regular);
    let main_window = NSWindow::new(
        CGRect::new(0.0, 0.0, 800.0, 600.0),
        vec![NSWindowStyleMask::Titled, NSWindowStyleMask::Closable, NSWindowStyleMask::Resizable]
    );
    app.activate(true);

    main_window.set_title("Hello, world!");
    main_window.make_key_and_order_front();

    let main_menu = NSMenu::new("Hello");
    let app_menu_item = NSMenuItem::new("Hello");
    let app_menu = NSMenu::new("Hello");
    app_menu.add_item(&NSMenuItem::new("Hello"));

    main_menu.add_item(&app_menu_item);

    app_menu_item.set_submenu(app_menu);

    app.set_main_menu(main_menu);

    app.run();
}