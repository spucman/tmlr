pub fn init(is_debug: bool) {
    fern::Dispatch::new()
        .format(|out, message, _record| {
            out.finish(format_args!(
                "{ts} {msg}",
                ts = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                msg = message
            ))
        })
        .level(if is_debug {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .chain(std::io::stdout())
        .apply()
        .expect("logging creation works");
}
