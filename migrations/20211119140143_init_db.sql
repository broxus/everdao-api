CREATE TABLE IF NOT EXISTS raw_transactions
(
    transaction      BYTEA   NOT NULL,
    transaction_hash BYTEA   NOT NULL,
    timestamp_block  INTEGER NOT NULL,
    timestamp_lt     BIGINT  NOT NULL,
    created_at       BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    PRIMARY KEY (transaction_hash)
);

CREATE TABLE IF NOT EXISTS proposals
(
    id                  INTEGER NOT NULL,
    address             VARCHAR NOT NULL,
    proposer            VARCHAR NOT NULL,
    description         VARCHAR NOT NULL,
    start_time          BIGINT  NOT NULL,
    end_time            BIGINT  NOT NULL,
    execution_time      BIGINT  NOT NULL,
    grace_period        BIGINT  NOT NULL,
    for_votes           NUMERIC NOT NULL,
    against_votes       NUMERIC NOT NULL,
    quorum_votes        NUMERIC NOT NULL,
    message_hash        BYTEA   NOT NULL,
    transaction_hash    BYTEA   NOT NULL,
    timestamp_block     INTEGER NOT NULL,
    actions             jsonb   NOT NULL,
    executed            BOOLEAN NOT NULL DEFAULT false,
    canceled            BOOLEAN NOT NULL DEFAULT false,
    queued              BOOLEAN NOT NULL DEFAULT false,
    executed_at         INTEGER NOT NULL,
    canceled_at         INTEGER NOT NULL,
    queued_at           INTEGER NOT NULL,
    updated_at          BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    created_at          BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS votes
(
    proposal_id         INTEGER NOT NULL,
    voter               VARCHAR NOT NULL,
    support             BOOLEAN NOT NULL,
    votes               NUMERIC NOT NULL,
    reason              VARCHAR NOT NULL,
    message_hash        BYTEA   NOT NULL,
    transaction_hash    BYTEA   NOT NULL,
    timestamp_block     INTEGER NOT NULL,
    created_at          BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    PRIMARY KEY (proposal_id, voter)
);
