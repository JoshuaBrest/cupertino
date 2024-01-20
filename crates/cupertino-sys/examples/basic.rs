use cupertino_sys::{
    appkit::{
        NSApplication, NSApplicationActivationPolicy, NSButton, NSFont, NSFontWeight,
        NSLayoutAnchorLike, NSLayoutConstraint, NSLayoutDimension, NSLayoutXAxisAnchor,
        NSLayoutYAxisAnchor, NSMenu, NSMenuItem, NSTextField, NSView, NSViewLike, NSWindow,
        NSWindowStyleMask,
    },
    core::CGRect,
};

fn main() {
    let app = NSApplication::new();
    app.set_activation_policy(NSApplicationActivationPolicy::Regular);
    let main_window = NSWindow::new(
        CGRect::new(0.0, 0.0, 800.0, 600.0),
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
    let main_view = NSView::new(CGRect::zero());
    main_window.set_content_view(&main_view);

    // Constraints
    let main_view_left = NSLayoutXAxisAnchor::left(&main_view);
    let main_view_right = NSLayoutXAxisAnchor::right(&main_view);
    let main_view_top = NSLayoutYAxisAnchor::top(&main_view);
    let main_view_bottom = NSLayoutYAxisAnchor::bottom(&main_view);
    let main_view_width = NSLayoutDimension::width(&main_view);
    let main_view_height = NSLayoutDimension::height(&main_view);

    // Create a textbox
    let heading = NSTextField::new(CGRect::zero());
    heading.set_draws_background(false);
    heading.set_bordered(false);
    heading.set_editable(false);
    heading.set_selectable(false);
    heading.set_string_value("Basic demo — Cupertino 🦀");
    heading.set_bezeled(false);
    heading.set_bordered(false);
    heading.set_font(NSFont::system_font(24.0, NSFontWeight::Bold));
    heading.disable_auto_layout();

    // Constraints
    let heading_left = NSLayoutXAxisAnchor::left(&heading);
    let heading_right = NSLayoutXAxisAnchor::right(&heading);
    let heading_top = NSLayoutYAxisAnchor::top(&heading);
    let heading_bottom = NSLayoutYAxisAnchor::bottom(&heading);

    // Create a textbox
    let text = NSTextField::new(CGRect::zero());
    text.set_draws_background(false);
    text.set_bordered(false);
    text.set_editable(false);
    text.set_selectable(false);
    text.set_string_value("This AppKit applcation was built with Cupertino in Rust 🦀.");
    text.set_bezeled(false);
    text.set_bordered(false);
    text.set_font(NSFont::system_font(14.0, NSFontWeight::Regular));
    text.disable_auto_layout();

    // Constraints
    let text_left = NSLayoutXAxisAnchor::left(&text);
    let text_right = NSLayoutXAxisAnchor::right(&text);
    let text_top = NSLayoutYAxisAnchor::top(&text);

    // Create a button
    let button = NSButton::new(CGRect::zero());
    button.set_title("Click me!");
    button.disable_auto_layout();

    // Constraints
    let button_right = NSLayoutXAxisAnchor::right(&button);
    let button_bottom = NSLayoutYAxisAnchor::bottom(&button);

    // Add the textbox to the main view
    main_view.add_subview(&button);
    main_view.add_subview(&heading);
    main_view.add_subview(&text);

    NSLayoutConstraint::activate_constraints(&[
        heading_left.anchor_eq(&main_view_left).constant(20.0),
        heading_right.anchor_eq(&main_view_right).constant(-20.0),
        heading_top.anchor_eq(&main_view_top).constant(20.0),
        text_left.anchor_eq(&main_view_left).constant(20.0),
        text_right.anchor_eq(&main_view_right).constant(-20.0),
        text_top.anchor_eq(&heading_bottom).constant(20.0),
        main_view_width.constant_eq(800.0),
        main_view_height.constant_eq(600.0),
        button_right.anchor_eq(&main_view_right).constant(-20.0),
        button_bottom.anchor_eq(&main_view_bottom).constant(-20.0),
    ]);

    app.run();
}
