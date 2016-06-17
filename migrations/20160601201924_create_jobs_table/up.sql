CREATE TABLE jobs (
  id SERIAL PRIMARY KEY,
  sha VARCHAR NOT NULL UNIQUE,
  running BOOLEAN NOT NULL DEFAULT FALSE,
  document_id INTEGER REFERENCES documents,
  CONSTRAINT running_document CHECK (running IS FALSE OR document_id IS NULL)
)
