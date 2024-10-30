// Mock storage for testing purposes
// use crate::serve_types::User;
// use rand::Rng;
// use rand::distributions::Alphanumeric;


pub fn is_logged_in() -> bool {
    true
}


// pub fn get_all_problems() -> Vec<Problem> {

//     let mut problems : Vec<Problem> = Vec::new();

//     for _ in 0..15 {
        
//         let random_number = rand::thread_rng().gen_range(1..101);
//         let levels = ["Easy", "Medium", "Hard"];
//         let tags = ["Array", "Hash Table", "String", "Math", "Two Pointers", "Binary Search", ];

//         let random_string: String = rand::thread_rng()
//                                         .sample_iter(&Alphanumeric)
//                                         .take(10)
//                                         .map(char::from)
//                                         .collect();
//         let random_date = format!("{}-{}-{}", rand::thread_rng().gen_range(2000..2022), rand::thread_rng().gen_range(1..13), rand::thread_rng().gen_range(1..29));

//         let problem = Problem {
//             id: random_number,
//             number: random_number,
//             name: random_string.clone(),
//             problem_statement: random_string.clone(),
//             hint: random_string.clone(),
//             tags: tags[rand::thread_rng().gen_range(0..5)].to_string(),
//             difficulty: levels[rand::thread_rng().gen_range(0..3)].to_string(),
//             solvers: rand::thread_rng().gen_range(1..100),
//             created_at: random_date.clone(),
//             updated_at: random_date.clone()
//         };

//         problems.push(problem)
//     }

//     problems
// }





// pub fn get_leaderboard_users() -> Vec<User> {

//     let mut users : Vec<User> = Vec::new();

//     for _ in 0..20 {
        
//         let random_number = rand::thread_rng().gen_range(1..101);

//         let random_string: String = rand::thread_rng()
//                                         .sample_iter(&Alphanumeric)
//                                         .take(10)
//                                         .map(char::from)
//                                         .collect();
//         let random_date = format!("{}-{}-{}", rand::thread_rng().gen_range(2000..2022), rand::thread_rng().gen_range(1..13), rand::thread_rng().gen_range(1..29));

//         let user = User {
//             id: random_number,
//             name: random_string.clone(),
//             email: format!("{}@gmail.com", random_string.clone()),
//             attempted: rand::thread_rng().gen_range(1..100),
//             solved: rand::thread_rng().gen_range(1..100),
//             rank: random_number,
//             created_at: random_date.clone(),
//             updated_at: random_date.clone()
//         };
//         users.push(user)
//     }

//     users
// }


// pub fn get_user_profile() -> User {
//     let random_number = rand::thread_rng().gen_range(1..101);

//     let random_string: String = rand::thread_rng()
//                                     .sample_iter(&Alphanumeric)
//                                     .take(10)
//                                     .map(char::from)
//                                     .collect();
//     let random_date = format!("{}-{}-{}", rand::thread_rng().gen_range(2000..2022), rand::thread_rng().gen_range(1..13), rand::thread_rng().gen_range(1..29));

//     let user = User {
//         id: random_number,
//         name: random_string.clone(),
//         email: format!("{}@gmail.com", random_string.clone()),
//         attempted: rand::thread_rng().gen_range(1..100),
//         solved: rand::thread_rng().gen_range(1..100),
//         rank: random_number,
//         created_at: random_date.clone(),
//         updated_at: random_date.clone()
//     };
    
//     user
// }

// pub fn get_problem() -> Problem {

//     let random_string: String = rand::thread_rng()
//                                     .sample_iter(&Alphanumeric)
//                                     .take(10)
//                                     .map(char::from)
//                                     .collect();
//     let random_date = format!("{}-{}-{}", rand::thread_rng().gen_range(2000..2022), rand::thread_rng().gen_range(1..13), rand::thread_rng().gen_range(1..29));

//     let random_number = rand::thread_rng().gen_range(1..101);
//     let levels = ["Easy", "Medium", "Hard"];
//     let tags = ["Array", "Hash Table", "String", "Math", "Two Pointers", "Binary Search", ];

//    let problem = Problem {
//         id: random_number,
//         number: random_number,
//         name: random_string.clone(),
//         problem_statement: random_string.clone(),
//         hint: random_string.clone(),
//         tags: tags[rand::thread_rng().gen_range(0..5)].to_string(),
//         difficulty: levels[rand::thread_rng().gen_range(0..3)].to_string(),
//         solvers: rand::thread_rng().gen_range(1..100),
//         created_at: random_date.clone(),
//         updated_at: random_date.clone()
//     };

//     problem
// }