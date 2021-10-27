use error_chain::error_chain;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        FromUtf8(::std::string::FromUtf8Error);
        Utf8(::std::str::Utf8Error);
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
        Json(serde_json::Error);
    }
}