-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;



-- application specific tables

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_users_username ON users (username);
CREATE INDEX idx_users_email ON users (email);


CREATE TABLE attempted_problems (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    problemId INT NOT NULL,
    is_solved BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_attempted_problems_user_id ON attempted_problems (user_id);
CREATE INDEX idx_attempted_problems_problemId ON attempted_problems (problemId);
CREATE INDEX idx_attempted_problems_is_solved ON attempted_problems (is_solved);


CREATE TABLE problems (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    problem_statement TEXT NOT NULL,
    tags TEXT NOT NULL,
    difficulty INT NOT NULL,
    hint TEXT,
    answer VARCHAR(255) NOT NULL,
    solved_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_problems_difficulty ON problems (difficulty);
CREATE INDEX idx_problems_solved_count ON problems (solved_count);


--- Basic Data

INSERT INTO problems (title, problem_statement, tags, difficulty, hint, answer) VALUES
('problem one', 'What is 2 + 2?', 'math', 1, 'Think about it', '4'),
('problem two', 'What is 3 + 3?', 'math', 1, 'Think about it', '6'),
('problem three', 'What is 4 + 4?', 'math', 1, 'Think about it', '8'),
('problem four', 'What is 5 + 5?', 'math', 1, 'Think about it', '10'),
('problem five', 'What is 6 + 6?', 'math', 1, 'Think about it', '12'),
('problem six', 'What is 7 + 7?', 'math', 1, 'Think about it', '14'),
('problem seven', 'What is 8 + 8?', 'math', 1, 'Think about it', '16'),
('problem eight', 'What is 9 + 9?', 'math', 1, 'Think about it', '18'),
('problem nine', 'What is 10 + 10?', 'math', 1, 'Think about it', '20'),
('problem ten', 'What is 11 + 11?', 'math', 1, 'Think about it', '22'),
('problem eleven', 'What is 12 + 12?', 'math', 1, 'Think about it', '24'),
('problem twelve', 'What is 13 + 13?', 'math', 1, 'Think about it', '26'),
('problem thirteen', 'What is 14 + 14?', 'math', 1, 'Think about it', '28'),
('problem fourteen', 'What is 15 + 15?', 'math', 1, 'Think about it', '30'),
('problem fifteen', 'What is 16 + 16?', 'math', 1, 'Think about it', '32'),
('problem sixteen', 'What is 17 + 17?', 'math', 1, 'Think about it', '34'),
('problem seventeen', 'What is 18 + 18?', 'math', 1, 'Think about it', '36'),
('problem eighteen', 'What is 19 + 19?', 'math', 1, 'Think about it', '38'),
('problem nineteen', 'What is 20 + 20?', 'math', 1, 'Think about it', '40'),
('problem twenty', 'What is 21 + 21?', 'math', 1, 'Think about it', '42'),
('problem twenty one', 'What is 22 + 22?', 'math', 1, 'Think about it', '44'),
('problem twenty two', 'What is 23 + 23?', 'math', 1, 'Think about it', '46'),
('problem twenty three', 'What is 24 + 24?', 'math', 1, 'Think about it', '48'),
('problem twenty four', 'What is 25 + 25?', 'math', 1, 'Think about it', '50'),
('problem twenty five', 'What is 26 + 26?', 'math', 1, 'Think about it', '52'),
('problem twenty six', 'What is 27 + 27?', 'math', 1, 'Think about it', '54'),
('problem twenty seven', 'What is 28 + 28?', 'math', 1, 'Think about it', '56'),
('problem twenty eight', 'What is 29 + 29?', 'math', 1, 'Think about it', '58'),
('problem twenty nine', 'What is 30 + 30?', 'math', 1, 'Think about it', '60'),
('problem thirty', 'What is 31 + 31?', 'math', 1, 'Think about it', '62'),
('problem thirty one', 'What is 32 + 32?', 'math', 1, 'Think about it', '64'),
('problem thirty two', 'What is 33 + 33?', 'math', 1, 'Think about it', '66'),
('problem thirty three', 'What is 34 + 34?', 'math', 1, 'Think about it', '68'),
('problem thirty four', 'What is 35 + 35?', 'math', 1, 'Think about it', '70'),
('problem thirty five', 'What is 36 + 36?', 'math', 1, 'Think about it', '72'),
('problem thirty six', 'What is 37 + 37?', 'math', 1, 'Think about it', '74'),
('problem thirty seven', 'What is 38 + 38?', 'math', 1, 'Think about it', '76'),
('problem thirty eight', 'What is 39 + 39?', 'math', 1, 'Think about it', '78'),
('problem thirty nine', 'What is 40 + 40?', 'math', 1, 'Think about it', '80'),
('problem forty', 'What is 41 + 41?', 'math', 1, 'Think about it', '82'),
('problem forty one', 'What is 42 + 42?', 'math', 1, 'Think about it', '84'),
('problem forty two', 'What is 43 + 43?', 'math', 1, 'Think about it', '86'),
('problem forty three', 'What is 44 + 44?', 'math', 1, 'Think about it', '88'),
('problem forty four', 'What is 45 + 45?', 'math', 1, 'Think about it', '90'),
('problem forty five', 'What is 46 + 46?', 'math', 1, 'Think about it', '92'),
('problem forty six', 'What is 47 + 47?', 'math', 1, 'Think about it', '94'),
('problem forty seven', 'What is 48 + 48?', 'math', 1, 'Think about it', '96'),
('problem forty eight', 'What is 49 + 49?', 'math', 1, 'Think about it', '98'),
('problem forty nine', 'What is 50 + 50?', 'math', 1, 'Think about it', '100'),
('problem fifty', 'What is 51 + 51?', 'math', 1, 'Think about it', '102'),
('problem fifty one', 'What is 52 + 52?', 'math', 1, 'Think about it', '104'),
('problem fifty two', 'What is 53 + 53?', 'math', 1, 'Think about it', '106'),
('problem fifty three', 'What is 54 + 54?', 'math', 1, 'Think about it', '108'),
('problem fifty four', 'What is 55 + 55?', 'math', 1, 'Think about it', '110'),
('problem fifty five', 'What is 56 + 56?', 'math', 1, 'Think about it', '112');



INSERT INTO users (username, email) VALUES
('user3', 'user3@gmail.com'),
('user4', 'user4@gmail.com'),
('user5', 'user5@gmail.com'),
('user6', 'user6@gmail.com'),
('user7', 'user7@gmail.com'),
('user8', 'user8@gmail.com'),
('user9', 'user9@gmail.com'),
('user10', 'user10@gmail.com'),
('user11', 'user11@gmail.com'),
('user12', 'user12@gmail.com'),
('user13', 'user13@gmail.com'),
('user14', 'user14@gmail.com'),
('user15', 'user15@gmail.com'),
('user16', 'user16@gmail.com'),
('user17', 'user17@gmail.com'),
('user18', 'user18@gmail.com'),
('user19', 'user19@gmail.com'),
('user20', 'user20@gmail.com'),
('user21', 'user21@gmail.com'),
('user22', 'user22@gmail.com'),
('user23', 'user23@gmail.com'),
('user24', 'user24@gmail.com'),
('user25', 'user25@gmail.com'),
('user26', 'user26@gmail.com'),
('user27', 'user27@gmail.com'),
('user28', 'user28@gmail.com'),
('user29', 'user29@gmail.com'),
('user30', 'user30@gmail.com'),
('user31', 'user31@gmail.com'),
('user32', 'user32@gmail.com'),
('user33', 'user33@gmail.com'),
('user34', 'user34@gmail.com'),
('user35', 'user35@gmail.com'),
('user36', 'user36@gmail.com'),
('user37', 'user37@gmail.com'),
('user38', 'user38@gmail.com'),
('user39', 'user39@gmail.com'),
('user40', 'user40@gmail.com'),
('user41', 'user41@gmail.com'),
('user42', 'user42@gmail.com'),
('user43', 'user43@gmail.com'),
('user44', 'user44@gmail.com'),
('user45', 'user45@gmail.com'),
('user46', 'user46@gmail.com'),
('user47', 'user47@gmail.com'),
('user48', 'user48@gmail.com'),
('user49', 'user49@gmail.com'),
('user50', 'user50@gmail.com'),
('user51', 'user51@gmail.com'),
('user52', 'user52@gmail.com'),
('user53', 'user53@gmail.com'),
('user54', 'user54@gmail.com'),
('user55', 'user55@gmail.com'),
('user56', 'user56@gmail.com'),
('user57', 'user57@gmail.com'),
('user58', 'user58@gmail.com'),
('user59', 'user59@gmail.com'),
('user60', 'user60@gmail.com'),
('user61', 'user61@gmail.com'),
('user62', 'user62@gmail.com'),
('user63', 'user63@gmail.com'),
('user64', 'user64@gmail.com'),
('user65', 'user65@gmail.com'),
('user66', 'user66@gmail.com'),
('user67', 'user67@gmail.com'),
('user68', 'user68@gmail.com'),
('user69', 'user69@gmail.com'),
('user70', 'user70@gmail.com'),
('user71', 'user71@gmail.com'),
('user72', 'user72@gmail.com'),
('user73', 'user73@gmail.com'),
('user74', 'user74@gmail.com'),
('user75', 'user75@gmail.com'),
('user76', 'user76@gmail.com'),
('user77', 'user77@gmail.com'),
('user78', 'user78@gmail.com'),
('user79', 'user79@gmail.com'),
('user80', 'user80@gmail.com'),
('user81', 'user81@gmail.com'),
('user82', 'user82@gmail.com'),
('user83', 'user83@gmail.com'),
('user84', 'user84@gmail.com'),
('user85', 'user85@gmail.com'),
('user86', 'user86@gmail.com'),
('user87', 'user87@gmail.com'),
('user88', 'user88@gmail.com'),
('user89', 'user89@gmail.com'),
('user90', 'user90@gmail.com'),
('user91', 'user91@gmail.com'),
('user92', 'user92@gmail.com'),
('user93', 'user93@gmail.com'),
('user94', 'user94@gmail.com'),
('user95', 'user95@gmail.com'),
('user96', 'user96@gmail.com'),
('user97', 'user97@gmail.com'),
('user98', 'user98@gmail.com'),
('user99', 'user99@gmail.com'),
('user100', 'user100@gmail.com'),
('user101', 'user101@gmail.com'),
('user102', 'user102@gmail.com');

