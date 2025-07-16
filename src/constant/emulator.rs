use std::sync::LazyLock;

pub static MUMU5: LazyLock<Vec<&str>> = LazyLock::new(|| {
    vec![
        "MuMuNxDevice.exe",
        "MuMuNxMain.exe",
        "MuMuVMMHeadless.exe",
        "MuMuVMMSVC.exe",
    ]
});
