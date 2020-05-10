CREATE TABLE comments (
    id INTEGER PRIMARY KEY,
    title TEXT,
    content TEXT NOT NULL,
    author_name TEXT,
    author_email TEXT,
    author_ip TEXT NOT NULL,
    dt TEXT NOT NULL,
    url TEXT NOT NULL
)

