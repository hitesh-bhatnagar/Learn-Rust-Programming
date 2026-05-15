use std::fmt;
use uuid::Uuid;
use chrono::DateTime;

pub enum TaskState {
    Pending,
    Inprogress,
    Completed,
}

// trait Display {
//     fn display(&self);
// }
// 
impl fmt::Display for TaskState{    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskState::Pending => write!(f, "Pending" ),
            TaskState::Inprogress => write!(f, "In progress"),
            TaskState::Completed => write!(f, "Completed"),
            
        }
    }
}

pub enum TaskPriority {
    Low,
    Medium,
    High,
}

impl fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskPriority::Low => write!(f, "Low"),
            TaskPriority::Medium => write!(f, "Medium"),
            TaskPriority::High => write!(f, "High"),
        }
    }
}

pub struct Task {
    id: Uuid,
    title: String,
    description: Option<String>,
    priority: TaskPriority,
    status: TaskState,
    due_date: Option<DateTime>,
    created_at: DateTime,
    updated_at: DateTime
}

impl fmt::Display for Task {
    fn fmt(&sekf, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match Self {
            Task::id => write!(f, "Id : "),
            Task::title => write!(f, "Title : "),
            Task::description => write!(f, "Description : "),
            Task::priority => write!(f, "Priority : "),
            Task::status => write!(f, "Status : "),
            Task::due_date => write!(f, "Due Date : "),
            Task::created_at => write!(f, "Created At : "),
            Task::updated_at=> write!(f, "Updated At : "),
            
        }
    }
}

