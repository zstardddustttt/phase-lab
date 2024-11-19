pub trait NumExtensions {
    fn seconds_to_msm(&self) -> String;
}

impl NumExtensions for f64 {
    fn seconds_to_msm(&self) -> String {
        let minutes = self / 60.0;
        let seconds = self % 60.0;
        let millis = (self * 1000.0) % 1000.0;

        format!(
            "{:02}:{:02}.{:03}",
            minutes as u32, seconds as u32, millis as u32
        )
    }
}

#[macro_export]
macro_rules! asset {
    ($($arg:tt)*) => {
        format!("{}", std::env::current_dir().unwrap().join("assets").join($($arg)*).display().to_string())
    };
}

#[macro_export]
macro_rules! image_asset {
    ($path:expr $(,)?) => {{
        eframe::egui::ImageSource::Bytes {
            uri: ::std::borrow::Cow::Borrowed(concat!("bytes://", crate::path_to_asset!($path))),
            bytes: eframe::egui::load::Bytes::Static(include_bytes!(crate::path_to_asset!($path))),
        }
    }};
}

#[macro_export]
macro_rules! path_to_asset {
    ($path:expr $(,)?) => {
        concat!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/"), $path)
    };
}
