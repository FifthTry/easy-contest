-- Your SQL goes here
CREATE TABLE IF NOT EXISTS ec_contest (
    id BIGSERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    start_at TIMESTAMP WITH TIME ZONE NOT NULL,
    end_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE IF NOT EXISTS ec_contest_submission (
    id BIGSERIAL PRIMARY KEY,
    -- we require fastn auth to be enabled for this to work
    user_id BIGINT REFERENCES fastn_user(id) ON DELETE CASCADE NOT NULL,
    title TEXT NOT NULL,
    deploy_url TEXT NOT NULL,
    source_url TEXT NOT NULL,
    message TEXT NOT NULL,
    is_winner BOOLEAN DEFAULT FALSE NOT NULL,
    contest_id BIGINT REFERENCES ec_contest(id) ON DELETE CASCADE NOT NULL
);
