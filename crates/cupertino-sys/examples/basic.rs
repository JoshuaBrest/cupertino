use cupertino_sys::{
    appkit::{
        NSApplication, NSApplicationActivationPolicy, NSButton, NSFont, NSFontWeight,
        NSLayoutAnchorLike, NSLayoutConstraint, NSLayoutDimension, NSLayoutXAxisAnchor,
        NSLayoutYAxisAnchor, NSMenu, NSMenuItem, NSTextField, NSView, NSViewLike, NSWindow,
        NSWindowStyleMask,
    },
    core::CGRect,
    foundation::NSString,
};
use objc2::sel;

fn main() {
    let app = NSApplication::new();
    app.set_activation_policy(NSApplicationActivationPolicy::Regular);
    let main_window = NSWindow::new(
        CGRect::new(0.0, 0.0, 800.0, 600.0),
        NSWindowStyleMask::TITLED
            | NSWindowStyleMask::CLOSABLE
            | NSWindowStyleMask::MINIATURIZABLE
            | NSWindowStyleMask::RESIZABLE,
    );
    app.activate(true);

    main_window.set_title(&NSString::new("Hello, world!"));
    main_window.make_key_and_order_front();

    // Menus
    let main_menu = NSMenu::new();
    let app_menu_item = NSMenuItem::new();
    let app_menu = NSMenu::new();
    let about_menu_item = NSMenuItem::new();
    let quit_menu_item = NSMenuItem::new();

    about_menu_item.set_title(&NSString::new("About"));
    about_menu_item.set_action(sel!(orderFrontStandardAboutPanel:));
    app_menu.add_item(&about_menu_item);

    quit_menu_item.set_title(&NSString::new("Quit"));
    quit_menu_item.set_action(sel!(terminate:));
    quit_menu_item.set_key_equivalent(&NSString::new("q"));

    app_menu.add_item(&quit_menu_item);
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

    // Create a textbox
    let heading = NSTextField::new(CGRect::zero());
    heading.set_draws_background(false);
    heading.set_bordered(false);
    heading.set_editable(false);
    heading.set_selectable(false);
    heading.set_string_value(&NSString::new("Basic demo — Cupertino 🦀"));
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
    text.set_string_value(&NSString::new(
        "This AppKit applcation was built with Cupertino in Rust 🦀.",
    ));
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
    button.set_title(&NSString::new("Click me!"));
    button.disable_auto_layout();

    // Constraints
    let button_right = NSLayoutXAxisAnchor::right(&button);
    let button_bottom = NSLayoutYAxisAnchor::bottom(&button);

    // Add the textbox to the main view
    main_view.add_subview(&button);
    main_view.add_subview(&heading);
    main_view.add_subview(&text);

    // Create constraints
    let constraint_heading_main_view_top = heading_top.anchor_eq(&main_view_top);
    let constraint_heading_main_view_left = heading_left.anchor_eq(&main_view_left);
    let constraint_heading_main_view_right = heading_right.anchor_eq(&main_view_right);
    let constraint_text_main_view_top = text_top.anchor_eq(&heading_bottom);
    let constraint_text_main_view_left = text_left.anchor_eq(&main_view_left);
    let constraint_text_main_view_right = text_right.anchor_eq(&main_view_right);
    let constraint_button_main_view_right = button_right.anchor_eq(&main_view_right);
    let constraint_button_main_view_bottom = button_bottom.anchor_eq(&main_view_bottom);

    // Add offsets to all constraints
    constraint_heading_main_view_top.set_constant(20.0);
    constraint_heading_main_view_left.set_constant(20.0);
    constraint_heading_main_view_right.set_constant(-20.0);
    constraint_text_main_view_top.set_constant(20.0);
    constraint_text_main_view_left.set_constant(20.0);
    constraint_text_main_view_right.set_constant(-20.0);
    constraint_button_main_view_right.set_constant(-20.0);
    constraint_button_main_view_bottom.set_constant(-20.0);

    // Activate all constraints

    NSLayoutConstraint::activate_constraints(&[
        constraint_heading_main_view_top,
        constraint_heading_main_view_left,
        constraint_heading_main_view_right,
        constraint_text_main_view_top,
        constraint_text_main_view_left,
        constraint_text_main_view_right,
        constraint_button_main_view_right,
        constraint_button_main_view_bottom,
    ]);

    app.run();
}
