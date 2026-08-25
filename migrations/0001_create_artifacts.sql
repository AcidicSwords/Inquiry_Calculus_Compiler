CREATE TABLE artifacts (
    artifact_ref BLOB NOT NULL PRIMARY KEY CHECK (length(artifact_ref) = 32),
    canonical_envelope BLOB NOT NULL
) STRICT, WITHOUT ROWID;
