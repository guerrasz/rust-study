use std::cmp::Ordering;

pub struct Job {
    pub salary: u32,
    pub time: u32,
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // can also be:
        // self.salary.partial_cmp(&other.salary);
        if self.salary == other.salary {
            Some(Ordering::Equal)
        } else if self.salary > other.salary {
            Some(Ordering::Greater)
        } else if self.salary < other.salary {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}
