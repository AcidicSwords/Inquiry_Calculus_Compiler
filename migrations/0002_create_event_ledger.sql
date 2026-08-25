CREATE TABLE event_ledger (
    ledger_sequence INTEGER PRIMARY KEY,
    event_ref BLOB NOT NULL UNIQUE CHECK (length(event_ref) = 32),
    ledger_parent BLOB CHECK (ledger_parent IS NULL OR length(ledger_parent) = 32)
) STRICT;
