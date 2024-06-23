-- Your SQL goes here
CREATE TABLE "posts" (
    "id"    INTEGER NOT NULL,
    "title" TEXT NOT NULL,
    "body"  TEXT NOT NULL,
    "published" INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY("id" AUTOINCREMENT)
)
