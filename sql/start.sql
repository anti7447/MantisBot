CREATE TABLE IF NOT EXISTS guilds (
  id BIGINT PRIMARY KEY,
  guildname VARCHAR,
  language VARCHAR(2) NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
  id BIGINT PRIMARY KEY,
  username VARCHAR
);

CREATE TABLE IF NOT EXISTS members (
  user_id BIGINT REFERENCES users(id),
  guild_id BIGINT REFERENCES guilds(id),
  CONSTRAINt members_pk PRIMARY KEY (user_id, guild_id)
);