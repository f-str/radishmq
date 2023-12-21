#[allow(dead_code)]
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type TopicType = String;
