use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;

pub fn init_logger() -> Result<(), fern::InitError> {
    Dispatch::new()
        .level(LevelFilter::Debug)
        .format(|out, msg, log_record| {
            out.finish(format_args!(
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                log_record.level(),
                msg,
            ))
        })
        .chain(fern::log_file("logs/debug.log")?)
        .apply()?;

    Ok(())
}