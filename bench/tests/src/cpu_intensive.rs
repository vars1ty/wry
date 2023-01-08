// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::process::exit;

fn main() -> wry::Result<()> {
  use wry::{
    application::{
      event::{Event, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::{Window, WindowBuilder},
    },
    http::{header::CONTENT_TYPE, Response},
    webview::WebViewBuilder,
  };

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().build(&event_loop).unwrap();

  let handler = |_window: &Window, req: String| {
    if &req == "process-complete" {
      exit(0);
    }
  };
  let _webview = WebViewBuilder::new(window)
    .unwrap()
    .with_custom_protocol("wrybench".into(), move |request| {
      let path = request.uri().to_string();
      let requested_asset_path = path.strip_prefix("wrybench://localhost").unwrap();
      let (data, mimetype): (_, String) = match requested_asset_path {
        "/index.css" => (
          include_bytes!("static/index.css").as_slice().into(),
          "text/css".into(),
        ),
        "/site.js" => (
          include_bytes!("static/site.js").as_slice().into(),
          "text/javascript".into(),
        ),
        "/worker.js" => (
          include_bytes!("static/worker.js").as_slice().into(),
          "text/javascript".into(),
        ),
        _ => (
          include_bytes!("static/index.html").as_slice().into(),
          "text/html".into(),
        ),
      };

      Response::builder()
        .header(CONTENT_TYPE, mimetype)
        .body(data)
        .map_err(Into::into)
    })
    .with_url("wrybench://localhost")?
    .with_ipc_handler(handler)
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      _ => {}
    }
  });
}
