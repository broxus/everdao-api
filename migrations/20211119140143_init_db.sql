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
    proposal_id      INTEGER   NOT NULL,
    proposer      VARCHAR   NOT NULL,
    description VARCHAR   NOT NULL,
    start_time  INTEGER NOT NULL,
    end_time  INTEGER NOT NULL,
    execution_time  INTEGER NOT NULL,
    for_votes     BIGINT  NOT NULL,
    against_votes     BIGINT  NOT NULL,
    quorum_votes     BIGINT  NOT NULL,
    state     VARCHAR  NOT NULL,
    updated_at       BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    created_at       BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    PRIMARY KEY (proposal_id)
);
