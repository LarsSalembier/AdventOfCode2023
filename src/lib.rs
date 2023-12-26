pub mod domain {
    pub mod common;
    pub mod error_handling;

    pub mod models {
        pub mod day;
        pub mod part;
    }
}

pub mod use_cases;

pub mod adapters {
    pub mod file_io;
}