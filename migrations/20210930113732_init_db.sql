-- Add migration script here
CREATE TABLE IF NOT EXISTS raw_transactions
(
    transaction      BYTEA   NOT NULL,
    transaction_hash BYTEA   NOT NULL,
    timestamp_block  INTEGER NOT NULL,
    timestamp_lt     BIGINT  NOT NULL,
    created_at       BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    PRIMARY KEY (transaction_hash)
);

CREATE INDEX IF NOT EXISTS raw_transactions_ix_timestamp_block ON raw_transactions (timestamp_block desc);

CREATE TABLE transactions
(
    message_hash     BYTEA   NOT NULL,
    transaction_hash BYTEA   NOT NULL,
    transaction_kind VARCHAR NOT NULL,
    user_address     VARCHAR NOT NULL,
    user_public_key  VARCHAR,
    bridge_exec      DECIMAL NOT NULL,
    timestamp_block  INTEGER NOT NULL,
    created_at       BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    PRIMARY KEY (message_hash, transaction_hash)
);

CREATE INDEX transactions_created_at ON transactions (created_at desc);
CREATE INDEX transactions_bridge_exec ON transactions (bridge_exec);
CREATE INDEX transactions_timestamp_block ON transactions (timestamp_block desc);

CREATE TABLE bridge_balances
(
    message_hash     BYTEA   NOT NULL,
    transaction_hash BYTEA   NOT NULL,
    transaction_kind VARCHAR NOT NULL,
    user_address     VARCHAR NOT NULL,
    user_balance     DECIMAL NOT NULL,
    reward           DECIMAL,
    bridge_balance   DECIMAL NOT NULL,
    stakeholders     INTEGER NOT NULL,
    average_apr      DECIMAL NOT NULL,
    bridge_reward    DECIMAL NOT NULL,
    timestamp_block  INTEGER NOT NULL,
    created_at       BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    PRIMARY KEY (message_hash, transaction_hash)
);

CREATE INDEX bridge_balances_created_at ON bridge_balances (created_at desc);
CREATE INDEX bridge_balances_timestamp_block ON bridge_balances (timestamp_block desc);
CREATE INDEX bridge_balances_user_address ON bridge_balances (user_address);

INSERT INTO bridge_balances
VALUES ('', '', 'Deposit', '', 0, null, 0, 0, 0, 0, 0);

CREATE TABLE user_keys
(
    user_address             VARCHAR NOT NULL,
    ton_pubkey               BYTEA   NOT NULL,
    ton_pubkey_is_confirmed  BOOL    NOT NULL,
    eth_address              BYTEA   NOT NULL,
    eth_address_is_confirmed BOOL    NOT NULL,
    until_frozen             INTEGER NOT NULL,
    updated_at               BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    created_at               BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    PRIMARY KEY (user_address)
);

CREATE TABLE unknown_user_keys
(
    address BYTEA   NOT NULL,
    kind    VARCHAR NOT NULL,
    PRIMARY KEY (address)
);

CREATE TABLE graph_data
(
    kind      VARCHAR NOT NULL,
    balance   DECIMAL NOT NULL,
    apr       DECIMAL NOT NULL,
    reward    DECIMAL NOT NULL,
    timestamp BIGINT  NOT NULL,
    PRIMARY KEY (timestamp, kind)
);

CREATE TABLE reward_rounds
(
    num_round     INTEGER NOT NULL,
    start_time    INTEGER NOT NULL,
    end_time      INTEGER NOT NULL,
    reward_tokens DECIMAL NOT NULL,
    total_reward  DECIMAL NOT NULL,
    PRIMARY KEY (num_round)
);

CREATE TABLE user_balances
(
    user_address  VARCHAR NOT NULL UNIQUE,
    user_kind     VARCHAR NOT NULL,
    stake_balance DECIMAL NOT NULL,
    frozen_stake  DECIMAL NOT NULL,
    last_reward   DECIMAL NOT NULL,
    total_reward  DECIMAL NOT NULL,
    until_frozen  INTEGER NOT NULL,
    updated_at    BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    created_at    BIGINT  NOT NULL DEFAULT extract(epoch from (CURRENT_TIMESTAMP(3) at time zone 'utc')) * 1000,
    PRIMARY KEY (user_address)
);

CREATE INDEX user_balances_created_at ON user_balances (created_at desc);
CREATE INDEX user_balances_updated_at ON user_balances (updated_at desc);
CREATE INDEX user_balances_stake_balance ON user_balances (stake_balance);
CREATE INDEX user_balances_frozen_stake ON user_balances (frozen_stake);
CREATE INDEX user_balances_last_reward ON user_balances (last_reward);
CREATE INDEX user_balances_total_reward ON user_balances (total_reward);
CREATE INDEX user_balances_total_until_frozen ON user_balances (until_frozen);
