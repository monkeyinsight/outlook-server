CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS history (
        "id" UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        "topic" VARCHAR(255) NOT NULL,
        "user" VARCHAR(255) NOT NULL,
        "action" VARCHAR(30),
        "date" TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );
