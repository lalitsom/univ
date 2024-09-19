pub use askama::Template;

// Define the Askama template for dynamic pages
#[derive(Template)]
#[template(path = "index.html")] // Specify the path to the HTML template
pub struct HomeTemplate<'a> {
    pub logged_in: bool,
    pub user_initials: &'a str
}

#[derive(Template)]
#[template(path = "problems.html")] // Specify the path to the HTML template
pub struct ProblemsTemplate<'a> {
    pub logged_in: bool,
    pub user_initials: &'a str,
    pub problems: Vec<Problem>
}

#[derive(Clone)]
pub struct Problem {
    pub id: i32,
    pub link: String,
    pub name: String,
    pub tags: String,
    pub solvers: i32,
    pub difficulty: String,
    pub solved: bool,
    pub date: String
}