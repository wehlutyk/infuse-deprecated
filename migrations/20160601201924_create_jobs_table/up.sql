CREATE TABLE jobs (
  id SERIAL PRIMARY KEY,
  sha VARCHAR NOT NULL,
  document_id INTEGER REFERENCES documents
)
