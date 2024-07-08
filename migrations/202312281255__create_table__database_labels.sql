CREATE TABLE "database_labels" (
    "id"         INTEGER PRIMARY KEY,
    "uri"        TEXT NOT NULL,
    "scope"      TEXT NOT NULL,
    "key"        TEXT NOT NULL,
    "value"      TEXT NOT NULL,
    "created_at" TEXT NOT NULL
) STRICT;
