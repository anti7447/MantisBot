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

CREATE TABLE IF NOT EXISTS selfroles (
  guild_id BIGINT REFERENCES guilds(id),
  channel_id BIGINT,
  message_id BIGINT,
  reaction VARCHAR,
  role_id BIGINT,
  active BOOLEAN NOT NULL,
  CONSTRAINt selfroles_pk PRIMARY KEY (guild_id, channel_id, message_id, reaction, role_id)
);