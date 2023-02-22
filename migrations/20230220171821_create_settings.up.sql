-- Add up migration script here
CREATE TABLE settings (
  id serial primary key,
  guild_id bigint not null,
  name text not null,
  value text not null,
  created_at timestamp not null default now(),
  updated_at timestamp not null default now()
);