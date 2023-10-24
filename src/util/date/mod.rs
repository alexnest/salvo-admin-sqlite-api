pub struct DateUtil;

impl DateUtil {
    pub fn get_current_datetime() -> chrono::DateTime<chrono::offset::Local> {
        return chrono::offset::Local::now();
    }

    pub fn get_current_common_datetime() -> String {
        return chrono::Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
    }
}
