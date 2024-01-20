use cupertino_sys::{
    appkit::{
        NSApplication, NSApplicationActivationPolicy, NSButton, NSFont, NSFontWeight,
        NSLayoutAnchorLike, NSLayoutXAxisAnchor, NSLayoutYAxisAnchor, NSMenu, NSMenuItem,
        NSTextField, NSView, NSViewLike, NSWindow, NSWindowStyleMask, NSLayoutConstraint,
    },
    core::CGRect,
};

fn main() {
    let app = NSApplication::new();
    app.set_activation_policy(NSApplicationActivationPolicy::Regular);
    let window_rect = CGRect::new(0.0, 0.0, 800.0, 600.0);
    let main_window = NSWindow::new(
        window_rect,
        vec![
            NSWindowStyleMask::Titled,
            NSWindowStyleMask::Closable,
            NSWindowStyleMask::Resizable,
        ],
    );
    app.activate(true);

    main_window.set_title("Hello, world!");
    main_window.make_key_and_order_front();

    // Menus
    let main_menu = NSMenu::new("");
    let app_menu_item = NSMenuItem::new("Hello");
    let app_menu = NSMenu::new("Hello");

    app_menu.add_item(&NSMenuItem::new("Hello"));
    main_menu.add_item(&app_menu_item);
    app_menu_item.set_submenu(app_menu);

    app.set_main_menu(main_menu);

    // Create a main view
    let main_view = NSView::new(window_rect);
    main_window.set_content_view(&main_view);

    // Constraints
    let main_view_left = NSLayoutXAxisAnchor::left(&main_view);
    let main_view_right = NSLayoutXAxisAnchor::right(&main_view);
    let main_view_top = NSLayoutYAxisAnchor::top(&main_view);
    let main_view_bottom = NSLayoutYAxisAnchor::bottom(&main_view);

    // Create a textbox
    let textbox = NSTextField::new(CGRect::new(0.0, 0.0, 0.0, 0.0));
    textbox.set_draws_background(false);
    textbox.set_bordered(false);
    textbox.set_editable(false);
    textbox.set_selectable(false);
    textbox.set_string_value("Crab rust 🦀🦀🦀🦀🦀🦀🦀🦀🦀");
    textbox.set_bezeled(false);
    textbox.set_font(NSFont::system_font(24.0, NSFontWeight::Bold));

    // Constraints
    let textbox_left = NSLayoutXAxisAnchor::left(&textbox);
    let textbox_right = NSLayoutXAxisAnchor::right(&textbox);
    let textbox_top = NSLayoutYAxisAnchor::top(&textbox);
    let textbox_bottom = NSLayoutYAxisAnchor::bottom(&textbox);

    NSLayoutConstraint::activate_constraints(&[
        textbox_left.anchor(&main_view_left, 0.0),
        textbox_right.anchor(&main_view_right, 0.0),
        textbox_top.anchor(&main_view_top, 0.0),
        textbox_bottom.anchor(&main_view_bottom, 0.0)
    ]);

    // Create a button
    let button = NSButton::new(CGRect::zero());
    button.set_title("Click me!");

    // Add the textbox to the main view
    main_view.add_subview(&button);

    main_view.add_subview(&textbox);

    app.run();
}
