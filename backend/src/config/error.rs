use error_chain;

/// controller -> service 字段类型
pub type RestResult<T> = std::result::Result<T, Error>;

error_chain::error_chain!(
    foreign_links {
        Io(std::io::Error);
        Rbatis(rbatis::Error);
    }
);
