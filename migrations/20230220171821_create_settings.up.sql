-- Add up migration script here
CREATE TABLE settings (
  guild_id bigint not null,
  name text not null,
  value jsonb not null,
  created_at timestamp not null default now(),
  updated_at timestamp not null default now()
);

CREATE UNIQUE INDEX settings_guild_id_name_unique ON settings (guild_id, name);
