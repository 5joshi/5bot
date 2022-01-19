CREATE TABLE discord_users (
    discord_id INT8 PRIMARY KEY,
    osu_name VARCHAR(15) NOT NULL
);

CREATE TABLE manual_links (
    discord_id INT8 PRIMARY KEY,
    osu_name VARCHAR(15) NOT NULL
);

CREATE TABLE messages (
    id INT8 NOT NULL PRIMARY KEY,
    guild_id INT8,
    channel_id INT8 NOT NULL,
    author INT8 NOT NULL,
    content TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    bot BOOL NOT NULL
);

CREATE TABLE unchecked_members (
    user_id INT8 NOT NULL PRIMARY KEY,
    joined TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE skins (
    username VARCHAR(15) NOT NULL PRIMARY KEY,
    entry TEXT NOT NULL
);

CREATE TABLE osuvs_maps (
    beatmap_id INT4 NOT NULL UNIQUE,
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (beatmap_id, start_date, end_date)
);

CREATE TABLE osuvs_scores (
    beatmap_id INT4 NOT NULL,
    user_id INT4 NOT NULL,
    mods INT4 NOT NULL,
    score JSON NOT NULL,
    FOREIGN KEY (beatmap_id) REFERENCES osuvs_maps(beatmap_id),
    PRIMARY KEY (beatmap_id, user_id, mods)
);

CREATE INDEX osuvs_scores_map_id ON osuvs_scores (beatmap_id);

CREATE TABLE osuvs_requests (
    beatmap_id INT4 NOT NULL PRIMARY KEY,
    beatmap JSON NOT NULL,
    requester INT8 NOT NULL
);