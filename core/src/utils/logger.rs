use std::{panic, env::current_dir, io::Write};
use backtrace::SymbolName;
use chrono::{Local, Timelike};
use env_logger::{Builder, WriteStyle, fmt::Formatter};
use log::{LevelFilter, Level, Record};

pub fn default_logger_format(buf: &mut Formatter, record: &Record) -> Result<(), std::io::Error> {
    let level = record.level();
    let args = record.args();
    if level == Level::Trace {
        writeln!(buf, "\n\x1b[35m{args}\x1b[0m")
    } else {
        let now = Local::now();
        let timestamp = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
        let module = match record.module_path() { Some(v)=>v, None=>"" };
        let line = match record.line() { Some(v)=>v, None=>0 };
        let styled_level = buf.default_styled_level(level);
        if level == Level::Error {
            writeln!(buf, "\x1b[90m{timestamp} {styled_level} \x1b[96m{module}:{line}\x1b[0m {args}\n{}",
                get_backtrace())
        }else {
            writeln!(buf, "\x1b[90m{timestamp} {styled_level} \x1b[96m{module}:{line}\x1b[0m {args}")
        }
    }
}

pub fn logger() -> env_logger::Builder {
    panic::set_hook(Box::new(|panic_info| log::error!("{panic_info}")));
    let mut b = Builder::new();
    b.filter(None, LevelFilter::Trace);
    b.filter(Some("wgpu_core"), LevelFilter::Info);
    b.filter(Some("wgpu_core::device"), LevelFilter::Warn);
    b.filter(Some("wgpu_hal"), LevelFilter::Info);
    b.filter(Some("naga"), LevelFilter::Info);
    b.format(default_logger_format);
    b.write_style(WriteStyle::Always);
    b
}

pub fn get_backtrace() -> String {
    let cur_dir = match current_dir() { Ok(v) => v, Err(e) => return e.to_string() };
    let mut res = String::new();
    for frame in backtrace::Backtrace::new().frames() {
        let symbol = &frame.symbols()[0];
        if let Some(path) = symbol.filename() {
            if !path.starts_with(&cur_dir) { continue }
        } else { continue };
        let name = symbol.name().unwrap_or(SymbolName::new(&[])).to_string();
        if name.starts_with("engine::logger") { continue; }
        res.push_str(&format!("{}:{}\n", name, symbol.lineno().unwrap_or_default()));
    }
    res
}