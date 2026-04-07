
pub struct Module {
    pub id: i64,
    pub mod_name: String,
}

pub struct Subject {
    pub id: i64,
    pub subj_name: String,
    pub mod_id: i64,
    
}
impl std::fmt::Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Module {}: {}. {}", self.mod_id, self.id, self.subj_name)
    }
}

pub struct Topic {
    pub id: i64,
    pub topic_name: String,
    pub subj_id: i64,
}

pub struct ColumnId {
    pub module_id: i64,
    pub subject_id: i64,
    pub topic_id: i64,
}

pub struct Randomized {
    pub module_id: i64,
    pub module_name: String,
    pub subject_id: i64,
    pub subj_name: String,
    pub topic_id: i64,
    pub topic_name: String,
}
impl std::fmt::Display for Randomized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Module: {}\nSubject: {}\nTopic: {}", self.module_name, self.subj_name, self.topic_name)
    }
}
impl Randomized {
    pub fn get_mod_id(&self) -> i64 {
        self.module_id
    }
    pub fn get_subj_id(&self) -> i64 {
        self.subject_id
    }
    pub fn get_topic_id(&self) -> i64 {
        self.topic_id
    }
}
