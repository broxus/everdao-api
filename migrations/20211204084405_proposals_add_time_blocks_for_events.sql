ALTER TABLE proposals ADD COLUMN canceled_at INTEGER;
ALTER TABLE proposals ADD COLUMN executed_at INTEGER;
ALTER TABLE proposals ADD COLUMN queued_at INTEGER;