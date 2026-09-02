use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Schedule {
    NoDeadline,
    Due(DateTime<Utc>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToDo {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub done: bool,
    pub schedule: Schedule,
}

impl ToDo {
    pub fn new(
        name: String,
        description: Option<String>,
        due_in_days: Option<i64>,
    ) -> Self {
        let schedule = match due_in_days {
            Some(days) => Schedule::Due(Utc::now() + Duration::days(days)),
            None => Schedule::NoDeadline,
        };

        Self {
            id: Utc::now().timestamp_millis() as u64,
            name,
            description,
            done: false,
            schedule,
        }
    }

    pub fn view(&self) {
        println!("----------------------------");
        println!("ID          : {}", self.id);
        println!("Name        : {}", self.name);
        println!("Description : {:?}", self.description);
        println!("Done        : {}", self.done);
        println!("Schedule    : {:?}", self.schedule);
        println!("----------------------------");
    }

    pub fn edit(
        &mut self,
        description: Option<String>,
        done: Option<bool>,
        due_in_days: Option<i64>,
    ) {
        if let Some(description) = description {
            self.description = Some(description);
        }

        if let Some(done) = done {
            self.done = done;
        }

        if let Some(days) = due_in_days {
            self.schedule = Schedule::Due(Utc::now() + Duration::days(days));
        }
    }
}