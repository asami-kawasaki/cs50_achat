-- Your SQL goes here
CREATE TABLE user
(
    id VARCHAR(100) NOT NULL PRIMARY KEY,
    nickname VARCHAR(20) NOT NULL,
    mail VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    profile_image VARCHAR(255) NOT NULL,
    delete_flag BOOLEAN,
    search_flag BOOLEAN,
    created_at INT NOT NULL,
    updated_at INT
);

CREATE TABLE follow
(
    id SERIAL PRIMARY KEY,
    to_user_id VARCHAR(100) NOT NULL,
    from_user_id VARCHAR(100) NOT NULL,
    direct_chat_room_id VARCHAR(255) NOT NULL,
    created_at INT NOT NULL,
    FOREIGN KEY(to_user_id) REFERENCES user(id),
    FOREIGN KEY(from_user_id) REFERENCES user(id),
    FOREIGN KEY(direct_chat_room_id) REFERENCES direct_chat_room(id)
);

CREATE TABLE direct_chat_room
(
    id SERIAL PRIMARY KEY,
    created_at INT NOT NULL
);

CREATE TABLE group_chat_room
(
    id SERIAL PRIMARY KEY,
    group_name VARCHAR(255) NOT NULL,
    group_image VARCHAR(255) NOT NULL,
    created_at INT NOT NULL,
    delete_flag BOOLEAN
);

CREATE TABLE message
(
    id SERIAL PRIMARY KEY,
    content_type_id INT NOT NULL,
    sender_id INT NOT NULL,
    direct_chat_room_id INT NOT NULL,
    group_chat_room_id INT NOT NULL,
    content TEXT NOT NULL,
    created_at INT NOT NULL,
    FOREIGN KEY(content_type_id) REFERENCES message_content_type(id),
    FOREIGN KEY(sender_id) REFERENCES user(id),
    FOREIGN KEY(direct_chat_room_id) REFERENCES direct_chat_room(id),
    FOREIGN KEY(group_chat_room_id) REFERENCES group_chat_room(id)
);

CREATE TABLE direct_member
(
    id SERIAL PRIMARY KEY,
    direct_chat_room_id INT NOT NULL,
    user_id VARCHAR(100) NOT NULL,
    delete_flag BOOLEAN,
    hidden_flag BOOLEAN,
    entry_date INT NOT NULL,
    last_read_time INT,
    FOREIGN KEY(direct_chat_room_id) REFERENCES direct_chat_room(id),
    FOREIGN KEY(user_id) REFERENCES user(id)
);

CREATE TABLE group_member
(
    id SERIAL PRIMARY KEY,
    group_chat_room_id INT NOT NULL,
    user_id VARCHAR(100) NOT NULL,
    delete_flag BOOLEAN,
    hidden_flag BOOLEAN,
    entry_date INT NOT NULL,
    last_read_time INT,
    FOREIGN KEY(group_chat_room_id) REFERENCES group_chat_room(id),
    FOREIGN KEY(user_id) REFERENCES user(id)
);

CREATE TABLE message_content_type
(
    id SERIAL PRIMARY KEY,
    content_type VARCHAR(20)
);
