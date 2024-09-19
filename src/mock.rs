use crate::serve_types::Problem;

pub fn is_logged_in() -> bool {
    true
}


pub fn user_initials() -> &'static str {
    "AB"
}

pub fn get_problems() -> Vec<Problem> {
    let problem = Problem {
        id: 1,
        link: "problem/1".to_string(),
        name: "Two Sum".to_string(),
        tags: "Array, Hash Table".to_string(),
        solvers: 100,
        difficulty: "Easy".to_string(),
        solved: true,
        date: "2021-09-01".to_string()
    };

    vec![problem; 50]
}