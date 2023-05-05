-- Postgres SQL

CREATE TABLE IF NOT EXISTS items (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    item_type VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS rewards (
    id SERIAL PRIMARY KEY,
    item_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (item_id) REFERENCES items (id)
);

CREATE TABLE IF NOT EXISTS quests (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    quest_type VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    objective TEXT NOT NULL,
    reward_id INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (reward_id) REFERENCES rewards (id)
);

CREATE TABLE IF NOT EXISTS missions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    objective TEXT NOT NULL,
    mission_type VARCHAR(255) NOT NULL,
    requirements JSON NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS quest_requirements (
    quest_id INTEGER NOT NULL,
    required_quest_id INTEGER,
    required_mission_id INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (quest_id) REFERENCES quests (id),
    FOREIGN KEY (required_quest_id) REFERENCES quests (id) ON DELETE SET NULL,
    FOREIGN KEY (required_mission_id) REFERENCES missions (id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS quests_missions (
    quest_id INTEGER NOT NULL,
    mission_id INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (quest_id) REFERENCES quests (id),
    FOREIGN KEY (mission_id) REFERENCES missions (id),
    PRIMARY KEY (quest_id, mission_id)
);

CREATE TABLE IF NOT EXISTS mission_steps (
    id SERIAL PRIMARY KEY,
    step_order INTEGER NOT NULL,
    mission_id INTEGER NOT NULL,
    status VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    step_image_url VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (mission_id) REFERENCES missions (id)
);
