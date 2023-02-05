use std::path::Path;

macro_rules! mime {
    ($var: ident, $ext: literal, $mime: literal) => {
        if $var.eq_ignore_ascii_case($ext) {
            return $mime;
        }
    };
}

pub fn get_content_type<P>(path: P) -> &'static str
where
    P: AsRef<Path>,
{
    let ext = path.as_ref().extension();
    match ext {
        Some(ext) => {
            mime!(ext, "css", "text/css");
            mime!(ext, "html", "text/html");
            mime!(ext, "jpg", "image/jpeg");
            mime!(ext, "jpeg", "image/jpeg");
            mime!(ext, "js", "text/javascript");
            mime!(ext, "md", "text/markdown");
            mime!(ext, "png", "image/png");
            mime!(ext, "svg", "image/svg+xml");
            mime!(ext, "txt", "text/plain");
            "application/octet-stream"
        }
        None => "application/octet-stream",
    }
}
