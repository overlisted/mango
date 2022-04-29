ALTER TABLE projects
  ADD COLUMN image VARCHAR;

CREATE TABLE tag_types (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  bg_color VARCHAR NOT NULL,
  fg_color VARCHAR NOT NULL
);

CREATE TABLE project_tags (
  id BIGSERIAL PRIMARY KEY,
  tag_type_id SERIAL REFERENCES tag_types(id),
  project_id VARCHAR(32) NOT NULL REFERENCES projects(id) ON UPDATE CASCADE
);

CREATE TABLE link_types (
  id SERIAL PRIMARY KEY,
  icon VARCHAR NOT NULL,
  bg_color VARCHAR NOT NULL,
  fg_color VARCHAR NOT NULL
);

CREATE TABLE project_links (
  id BIGSERIAL PRIMARY KEY,
  url VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  link_type_id SERIAL REFERENCES link_types(id),
  project_id VARCHAR(32) NOT NULL REFERENCES projects(id) ON UPDATE CASCADE
);
