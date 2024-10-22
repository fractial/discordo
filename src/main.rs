#![windows_subsystem = "windows"]

use std::fs;

fn main() -> wry::Result<()> {
    use std::env;
    use tao::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };
    use wry::WebViewBuilder;

    let args: Vec<String> = env::args().collect();
    let url = match args.get(1) {
        Some(arg) => arg.clone(),
        None => "https://discord.com/channels/@me".to_string(),
    };

    let js = fs::read_to_string("script.js").unwrap_or_else(|_| {
        println!("`script.js` not found, using fallback!");
        "".to_string()
    });

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("")
        .build(&event_loop)
        .unwrap();
    let _webview = WebViewBuilder::new()
        .with_url(url)
        .with_initialization_script(js.as_str())
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
