use ic_cdk::api::time;

pub fn generate_unique_id() -> String {
    format!("{}", time())
}
