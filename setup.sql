CREATE TABLE doctypes (
  id SERIAL PRIMARY KEY,
  name VARCHAR,
  public_id VARCHAR,
  system_id VARCHAR
);

CREATE TABLE elements (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  comment VARCHAR NOT NULL
);
