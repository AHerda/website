use std::sync::RwLock;

use crate::components::project::Project;

pub const PROJECTS: RwLock<[Project; 4]> = RwLock::new([
    Project::new(
        "Checkers",
        "Simple project in java to learn the basics of programming.",
        Some("https://github.com/AHerda/warcaby"),
    ),
    Project::new(
        "Radio Traffic Analyzer",
        "Engineering thesis project. Developed in C++.",
        Some("https://github.com/AHerda/sdr-analyzer"),
    ),
    Project::new(
        "Website",
        "Website written in rust using actix-web and maud.",
        Some("https://github.com/AHerda/website"),
    ),
    Project::new(
        "Compiler",
        "Simple compiler written in C++.",
        Some("https://github.com/AHerda/Kompilator"),
    ),
]);
