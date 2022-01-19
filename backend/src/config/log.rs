use crate::service::CONTEXT;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::{RollingType, Packer};
use std::time::Duration;
use log::Level;
use fast_log::plugin::packer::{LZ4Packer, ZipPacker, GZipPacker, LogPacker};

pub fn init_log() {
    // 创建文件夹路径
    std::fs::create_dir_all(&CONTEXT.config.log_dir);
    // 初始化
    fast_log::init_split_log(
        &CONTEXT.config.log_dir,
        str_to_max_temp_size(&CONTEXT.config.log_temp_size),
        str_to_rolling_type(&CONTEXT.config.log_rolling_type),
        str_to_log_level(&CONTEXT.config.log_level),
        None,
        choose_packer(&CONTEXT.config.log_pack_compress),
        true
    );
}

// 最大分流值
fn str_to_max_temp_size(size: &str) -> LogSize {
    match size {
        size if size.ends_with("GB") => {
            let end = size.find("GB").unwrap();
            let num = size[0..end].to_string();
            LogSize::GB(num.parse::<usize>().unwrap())
        }
        size if size.ends_with("MB") => {
            let end = size.find("MB").unwrap();
            let num = size[0..end].to_string();
            LogSize::MB(num.parse::<usize>().unwrap())
        }
        size if size.ends_with("KB") => {
            let end = size.find("KB").unwrap();
            let num = size[0..end].to_string();
            LogSize::KB(num.parse::<usize>().unwrap())
        }
        _ => LogSize::MB(100),
    }
}

fn str_to_rolling_type(rolling: &str) -> RollingType {
    match rolling {
        rolling if rolling.starts_with("KeepNum(") => {
            let end = rolling.find(")").unwrap();
            let num = rolling["KeepNum(".len()..end].to_string();
            RollingType::KeepNum(num.parse::<i64>().unwrap())
        }
        rolling if rolling.starts_with("KeepTime(") => {
            let end = rolling.find(")").unwrap();
            let num = rolling["KeepTime(".len()..end].to_string();
            RollingType::KeepTime(Duration::from_secs(num.parse::<u64>().unwrap()))
        }
        _ => RollingType::All,
    }
}

fn str_to_log_level(level: &str) -> Level {
    match level {
        "warn" => Level::Warn,
        "error" => Level::Error,
        "trace" => Level::Trace,
        "info" => Level::Info,
        "debug" => Level::Debug,
        _ => Level::Info,
    }
}

fn choose_packer(packer: &str) -> Box<dyn Packer> {
    match packer {
        "lz4" => Box::new(LZ4Packer {}),
        "zip" => Box::new(ZipPacker {}),
        "gzip" => Box::new(GZipPacker {}),
        _ => Box::new(LogPacker {}),
    }
}