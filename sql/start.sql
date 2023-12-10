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
  CONSTRAINT members_pk PRIMARY KEY (user_id, guild_id)
);

CREATE TABLE IF NOT EXISTS selfroles (
  id int not null,
  guild_id BIGINT REFERENCES guilds(id),
  channel_id BIGINT NOT NULL,
  message_id BIGINT NOT NULL,
  reaction VARCHAR NOT NULL,
  role_id BIGINT NOT NULL,
  active BOOLEAN NOT NULL,
  CONSTRAINT id_guildid UNIQUE(id, guild_id),
  CONSTRAINT selfroles_pk PRIMARY KEY (guild_id, channel_id, message_id, reaction, role_id)
);