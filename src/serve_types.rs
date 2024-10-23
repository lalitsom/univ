pub use askama::Template;

use std::sync::Mutex;
use oauth2::basic::BasicClient;

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
}

#[derive(Template)]
#[template(path = "profile.html")] // Specify the path to the HTML template
pub struct ProfileTemplate {
    pub logged_in: bool,
    pub user: User,
}

// storage types for the mock data

#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub attempted: i32,
    pub solved: i32,
    pub rank: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone)]
pub struct Problem {
    pub id: i32,
    pub number: i32,
    pub name: String,
    pub description: String,
    pub hint: String,
    pub tags: String,
    pub difficulty: String,
    pub solvers: i32,
    pub created_at: String,
    pub updated_at: String,
}

pub struct AppState {
    pub oauth_client: Mutex<BasicClient>,
}
