CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    mail TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS folders (
    id SERIAL PRIMARY KEY,
    user_id SERIAL NOT NULL,
    name TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS tasks {
    id SERIAL PRIMARY KEY,
    folder_id SERIAL NOT NULL,
    content TEXT NOT NULL DEFAULT "",
    priority INTEGER,
    FOREIGN KEY (folder_id) REFERENCES folders(id)
}