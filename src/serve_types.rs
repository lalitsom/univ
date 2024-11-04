use crate::storage::model::{Problem, User};
pub use askama::Template;

// Define the Askama template for dynamic pages
#[derive(Template)]
#[template(path = "index.html")] // Specify the path to the HTML template
pub struct HomeTemplate {
    pub logged_in: bool,
}

#[derive(Template)]
#[template(path = "problems.html")] // Specify the path to the HTML template
pub struct ProblemsTemplate {
    pub logged_in: bool,
    pub problems: Vec<Problem>,
}

#[derive(Template)]
#[template(path = "leaderboard.html")] // Specify the path to the HTML template
pub struct LeaderboardTemplate {
    pub logged_in: bool,
    pub users: Vec<User>,
}

#[derive(Template)]
#[template(path = "problem.html")] // Specify the path to the HTML template
pub struct ProblemTemplate {
    pub logged_in: bool,
    pub problem: Problem,
    pub already_solved: bool,
}

#[derive(Template)]
#[template(path = "profile.html")] // Specify the path to the HTML template
pub struct ProfileTemplate {
    pub logged_in: bool,
    pub user: User,
}