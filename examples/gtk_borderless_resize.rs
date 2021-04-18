// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#[cfg(not(target_os = "linux"))]
fn main() {}

#[cfg(target_os = "linux")]
fn main() -> wry::Result<()> {
  use gdk::EventButton;
  use gio::{prelude::*, Cancellable};
  use gtk::prelude::*;
  use gtk::ApplicationWindow;

  fn hit_test(window: &ApplicationWindow, event: &EventButton) -> gdk::WindowEdge {
    let (cx, cy) = event.get_root();
    let (left, top) = window.get_position();
    let (w, h) = window.get_size();
    let (right, bottom) = (left + w, top + h);
    let fake_border = 5; // change this to manipulate how far inside the window, the resize can happen

    const LEFT: i32 = 00001;
    const RIGHT: i32 = 0b0010;
    const TOP: i32 = 0b0100;
    const BOTTOM: i32 = 0b1000;
    const TOPLEFT: i32 = TOP | LEFT;
    const TOPRIGHT: i32 = TOP | RIGHT;
    const BOTTOMLEFT: i32 = BOTTOM | LEFT;
    const BOTTOMRIGHT: i32 = BOTTOM | RIGHT;

    let result = LEFT
      * (if (cx as i32) < (left + fake_border) {
        1
      } else {
        0
      })
      | RIGHT
        * (if (cx as i32) >= (right - fake_border) {
          1
        } else {
          0
        })
      | TOP
        * (if (cy as i32) < (top + fake_border) {
          1
        } else {
          0
        })
      | BOTTOM
        * (if (cy as i32) >= (bottom - fake_border) {
          1
        } else {
          0
        });

    match result {
      LEFT => gdk::WindowEdge::West,
      RIGHT => gdk::WindowEdge::East,
      TOP => gdk::WindowEdge::North,
      BOTTOM => gdk::WindowEdge::South,
      TOPLEFT => gdk::WindowEdge::NorthWest,
      TOPRIGHT => gdk::WindowEdge::NorthEast,
      BOTTOMLEFT => gdk::WindowEdge::SouthWest,
      BOTTOMRIGHT => gdk::WindowEdge::SouthEast,
      // has to be bigger than 7. otherwise it will match the number with a variant of gdk::WindowEdge enum and we don't want to do that
      _ => gdk::WindowEdge::__Unknown(8),
    }
  }

  gtk::init()?;
  let app = gtk::Application::new(Some("org.tauri.demo"), gio::ApplicationFlags::FLAGS_NONE)?;
  let cancellable: Option<&Cancellable> = None;
  app.register(cancellable)?;

  let window = gtk::ApplicationWindow::new(&app);
  window.set_default_size(320, 200);
  window.set_title("Basic example");
  window.show_all();
  window.set_decorated(false);

  window.connect_motion_notify_event(|_window, event| {
    //TODO: use the hit-test function and apply cursor style
    Inhibit(false)
  });

  window.connect_button_press_event(|window, event| {
    if event.get_button() == 1 {
      let (cx, cy) = event.get_root();

      window.begin_resize_drag(
        hit_test(window, event),
        event.get_button() as i32,
        cx as i32,
        cy as i32,
        event.get_time(),
      );
    }
    Inhibit(false)
  });

  loop {
    gtk::main_iteration();
  }
}
