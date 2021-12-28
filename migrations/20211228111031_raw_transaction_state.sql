DROP TYPE IF EXISTS raw_transaction_state_type;

CREATE TYPE raw_transaction_state_type as ENUM (
    'Idle',
    'Fail',
    'Success',
    'InProgress'
    );

ALTER TABLE raw_transactions ADD COLUMN state raw_transaction_state_type NOT NULL DEFAULT 'Idle';
