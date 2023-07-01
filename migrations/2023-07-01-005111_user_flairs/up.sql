-- Your SQL goes here
CREATE TABLE user_flairs (
    id SERIAL PRIMARY KEY,
    flair_id INTEGER,
    community_id INTEGER,
    created_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (flair_id) REFERENCES flairs(id),
    FOREIGN KEY (community_id) REFERENCES community(id)
);
