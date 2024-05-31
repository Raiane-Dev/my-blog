CREATE TABLE IF NOT EXISTS posts (
    id SERIAL NOT NULL PRIMARY KEY,
    title TEXT NOT NULL, 
    body TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id SERIAL NOT NULL PRIMARY KEY,
    username VARCHAR(255) NOT NULL, 
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    role VARCHAR(50)
);

CREATE TABLE IF NOT EXISTS post_cover (
    id SERIAL NOT NULL PRIMARY KEY,
    post_id INT NOT NULL,
    headline VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    image_path VARCHAR(255) NOT NULL,
    CONSTRAINT fk_post_id FOREIGN KEY(post_id) REFERENCES posts(id)
);

INSERT INTO post_cover (post_id, headline, description, image_path) VALUES (1, "Test", "test description", "/assets/mylogo.png")