use pretend_core::{App, Presence};

pub fn print_start(presence: &Presence) {
    println\!("Started activity for {}", presence.application_id);
    if let Some(details) = &presence.details {
        println\!("details: {}", details);
    }
    if let Some(state) = &presence.state {
        println\!("state: {}", state);
    }
}

pub fn print_stop() {
    println\!("Stopped Rich Presence activity.");
}

pub fn print_apps(apps: &[App]) {
    if apps.is_empty() {
        println\!("No applications found.");
        return;
    }

    for app in apps {
        println\!("{} ({})", app.name, app.application_id);
        if \!app.aliases.is_empty() {
            println\!("  aliases: {}", app.aliases.join(", "));
        }
    }
}

pub fn print_config(summary: &str) {
    println\!("{}", summary);
}
