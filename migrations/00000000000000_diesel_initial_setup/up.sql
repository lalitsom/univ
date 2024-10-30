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
('problem ab', 'What is 3 + 3?', 'math', 1, 'Think about it', '6'),
('problem por', 'What is 4 + 4?', 'math', 1, 'Think about it', '8');


INSERT INTO users (username, email) VALUES
('user1', 'user1@gmail.com'),
('user2', 'user22@gmail.com');