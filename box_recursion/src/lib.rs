#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None,}
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Worker {
            role,
            name,
            next: self.grade.take()
        };
        self.grade = Some(Box::new(new_worker));
    }
    pub fn remove_worker(&mut self) -> Option<String> {
        self.grade.take().map(|worker| {
            let name = worker.name;
            self.grade = worker.next;
            return Some(name)
        });
        None
    }
    
    pub fn last_worker(&self) -> Option<(String, String)> {
        let mut current = self.grade.as_ref()?;
        Some((current.name.clone(), current.role.clone()))
    }

}
