DROP TABLE IF EXISTS todo_list;
DROP TABLE IF EXISTS todo_item;

CREATE TABLE todo_list (
    id SERIAL PRIMARY KEY,
    title VARCHAR(150) NOT NULL
);

CREATE TABLE todo_item (
    id SERIAL PRIMARY KEY,
    title VARCHAR(150) NOT NULL,
    checked BOOLEAN NOT NULL DEFAULT false,
    list_id INTEGER NOT NULL,
    FOREIGN KEY (list_id) REFERENCES todo_list(id)
) ;

INSERT INTO todo_list (title) VALUES ('List 1'), ('List 2');
INSERT INTO todo_item (title, list_id) VALUES ('Item 1', 1), ('Item 2', 1), ('Item 1', 2);