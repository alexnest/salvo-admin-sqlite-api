use uuid::Uuid;

pub struct MyUuid;

impl MyUuid {
    pub fn new() -> String {
        return Uuid::new_v4().to_string().replace("-", "");
    }
}
