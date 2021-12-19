CREATE INDEX proposals_address_idx ON proposals (address);
CREATE INDEX proposals_proposer_idx ON proposals (proposer);
CREATE INDEX proposals_start_time_idx ON proposals (start_time);
CREATE INDEX proposals_end_time_idx ON proposals (end_time);
CREATE INDEX proposals_timestamp_block_idx ON proposals (timestamp_block);

CREATE INDEX votes_proposal_id_idx ON votes (proposal_id);
CREATE INDEX votes_voter_idx ON votes (voter);
CREATE INDEX votes_support_idx ON votes (support);
CREATE INDEX votes_locked_idx ON votes (locked);
CREATE INDEX votes_timestamp_block_idx ON votes (timestamp_block);
