use serde::{Deserialize, Serialize};

///服务启动配置
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    /// 数据库连接地址
    pub db_url: String,

    /// 日志目录 "target/logs/"
    pub log_dir: String,
    /// "100MB" 日志分割尺寸-单位KB,MB,GB
    pub log_temp_size: String,
    /// 日志打包格式可选“”（空-不压缩）“gzip”（gz压缩包）“zip”（zip压缩包）“lz4”（lz4压缩包（非常快））
    pub log_pack_compress: String,
    ///日志滚动配置   保留全部:All,按时间保留:KeepTime(Duration),按版本保留:KeepNum(i64)
    pub log_rolling_type: String,
    ///日志等级
    pub log_level: String,

}

///默认配置路径
impl Default for AppConfig {
    fn default() -> Self {
        // 读取基础配置文件
        let yml_data = include_str!("../../application.yml");
        let data_result: AppConfig = serde_yaml::from_str(yml_data).expect("读取环境文件异常");

        data_result
    }
}