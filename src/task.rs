pub mod task {
    use std::fmt;
    
    use serde::{
        Deserialize,
        Serialize
    };

    #[derive(Debug, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        Completed,
    }

    impl fmt::Display for Status {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                Status::Pending => write!(f, "Pending"),
                Status::Completed => write!(f, "Completed"),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Task {
        pub id: u32,
        pub description: String,
        pub status: Status,
    }

    impl fmt::Display for Task {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[{}] {} - {}", self.id, self.status, self.description)
        }
    }
}