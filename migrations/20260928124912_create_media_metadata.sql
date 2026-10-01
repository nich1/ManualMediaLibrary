-- Add migration script here
CREATE TYPE upload_status AS ENUM (
    'uploading',
    'finished',
    'failed'
);

CREATE TABLE media_metadata (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    filename TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    duration_ms BIGINT NOT NULL,
    status upload_status NOT NULL
);