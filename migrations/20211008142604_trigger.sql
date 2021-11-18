-- Add migration script here
create function new_graph_data_log() returns trigger
    language plpgsql
as
$$
BEGIN
    INSERT INTO graph_data (kind, apr, balance, reward, timestamp)
    VALUES ('H1',
            NEW.average_apr,
            NEW.bridge_balance,
            NEW.bridge_reward,
            EXTRACT(epoch FROM date_trunc('hour', to_timestamp(NEW.timestamp_block))) * 1000)
    ON CONFLICT (timestamp, kind) DO UPDATE
        SET apr     = NEW.average_apr,
            balance = NEW.bridge_balance,
            reward  = NEW.bridge_reward
    WHERE graph_data.timestamp = EXTRACT(epoch FROM date_trunc('hour', to_timestamp(NEW.timestamp_block))) * 1000
      AND graph_data.kind = 'H1';

    INSERT INTO graph_data (kind, apr, balance, reward, timestamp)
    VALUES ('D1',
            NEW.average_apr,
            NEW.bridge_balance,
            NEW.bridge_reward,
            EXTRACT(epoch FROM date_trunc('day', to_timestamp(NEW.timestamp_block))) * 1000)
    ON CONFLICT (timestamp, kind) DO UPDATE
        SET apr     = NEW.average_apr,
            balance = NEW.bridge_balance,
            reward  = NEW.bridge_reward
    WHERE graph_data.timestamp = EXTRACT(epoch FROM date_trunc('day', to_timestamp(NEW.timestamp_block))) * 1000
      AND graph_data.kind = 'D1';
    return NEW;
END;
$$;

create trigger new_pair_ohlcv_log
    after insert
    on bridge_balances
    for each row
execute procedure new_graph_data_log();