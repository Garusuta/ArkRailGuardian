pub const LOCAL_HOST: &str = "127.0.0.1";
pub const GET_FOREGROUND_APP: &str =
    "dumpsys window | grep mCurrentFocus | sed 's/ /\n/g' | grep '/' | cut -d '/' -f1";
