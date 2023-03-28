-- Add up migration script here
CREATE TABLE welcome (
    guild_id BIGINT NOT NULL,
    channel_id BIGINT,
    message TEXT,
    active BOOLEAN DEFAULT false,
    PRIMARY KEY (guild_id)
);
