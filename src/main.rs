// Minimal example wrapper for argument validation before calling openaction::init_plugin
use std::env;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // If no args are provided, exit cleanly (this avoids installer panic)
    if args.len() <= 1 {
        println!("opendeck-mirabox293: no args provided; exiting (installer check).");
        return Ok(());
    }

    // Simple helper to get the next value after a flag (like -port)
    let mut get_flag = |flag: &str| -> Option<String> {
        args.iter()
            .position(|x| x.eq_ignore_ascii_case(flag))
            .and_then(|pos| args.get(pos + 1))
            .cloned()
    };

    let port = match get_flag("-port") {
        Some(p) => p,
        None => {
            eprintln!("Missing required flag: -port");
            std::process::exit(2);
        }
    };

    let plugin_uuid = match get_flag("-pluginUUID") {
        Some(u) => u,
        None => {
            eprintln!("Missing required flag: -pluginUUID");
            std::process::exit(2);
        }
    };

    let register_event = match get_flag("-registerEvent") {
        Some(e) => e,
        None => {
            eprintln!("Missing required flag: -registerEvent");
            std::process::exit(2);
        }
    };

    // Now call into the existing init function (adapt the handlers below to your code)
    // Replace the placeholders with your actual handlers that you previously used.
    openaction::init_plugin(
        /* global_event_handler: */ /* your global handler value */,
        /* action_event_handler: */ /* your action handler value */,
    )
    .await?;

    Ok(())
}
