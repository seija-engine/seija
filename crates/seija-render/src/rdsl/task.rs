#[derive(Debug)]
pub enum ScriptTask {
    AddUniform(String),
    SelectAddUniform(String,String),
}

pub struct TaskContext {
    pub list:Vec<ScriptTask>
}

impl TaskContext {
    pub fn new() -> Self {
        TaskContext { list:vec![] }
    }

    pub fn add_task(&mut self,task:ScriptTask) {
        self.list.push(task);
    }
}