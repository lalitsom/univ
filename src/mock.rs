use crate::serve_types::Problem;
use crate::serve_types::User;
use rand::Rng;
use rand::distributions::Alphanumeric;


pub fn is_logged_in() -> bool {
    true
}


pub fn user_initials() -> &'static str {
    "AB"
}

pub fn get_problems() -> Vec<Problem> {

    let mut problems : Vec<Problem> = Vec::new();

    for _ in 0..15 {
        
        let random_number = rand::thread_rng().gen_range(1..101);
        let levels = ["Easy", "Medium", "Hard"];
        let tags = ["Array", "Hash Table", "String", "Math", "Two Pointers", "Binary Search", ];

        let random_string: String = rand::thread_rng()
                                        .sample_iter(&Alphanumeric)
                                        .take(10)
                                        .map(char::from)
                                        .collect();
        let random_date = format!("{}-{}-{}", rand::thread_rng().gen_range(2000..2022), rand::thread_rng().gen_range(1..13), rand::thread_rng().gen_range(1..29));

        let problem = Problem {
            id: random_number,
            link: format!("{}{}", "problem/", random_number),
            name: random_string,
            tags: tags[rand::thread_rng().gen_range(0..5)].to_string(),
            solvers: rand::thread_rng().gen_range(1..100),
            difficulty: levels[rand::thread_rng().gen_range(0..3)].to_string(),
            solved: true,
            date: random_date
        };

        problems.push(problem)
    }

    problems
}





pub fn get_users() -> Vec<User> {

    let mut users : Vec<User> = Vec::new();

    for _ in 0..20 {
        
        let random_number = rand::thread_rng().gen_range(1..101);

        let random_string: String = rand::thread_rng()
                                        .sample_iter(&Alphanumeric)
                                        .take(10)
                                        .map(char::from)
                                        .collect();
        let random_date = format!("{}-{}-{}", rand::thread_rng().gen_range(2000..2022), rand::thread_rng().gen_range(1..13), rand::thread_rng().gen_range(1..29));

        let user = User {
            rank: random_number,
            name: random_string, 
            solved: rand::thread_rng().gen_range(1..100),
            join_date: random_date
        };

        users.push(user)
    }

    users
}