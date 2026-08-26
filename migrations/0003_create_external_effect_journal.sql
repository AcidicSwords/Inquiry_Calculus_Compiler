CREATE TABLE external_effect_journal (
    dispatch_token BLOB NOT NULL PRIMARY KEY CHECK (length(dispatch_token) = 32),
    request_ref BLOB NOT NULL CHECK (length(request_ref) = 32),
    operator_ref BLOB NOT NULL CHECK (length(operator_ref) = 32),
    ledger_parent BLOB CHECK (ledger_parent IS NULL OR length(ledger_parent) = 32),
    completed_event BLOB CHECK (completed_event IS NULL OR length(completed_event) = 32)
) STRICT, WITHOUT ROWID;
