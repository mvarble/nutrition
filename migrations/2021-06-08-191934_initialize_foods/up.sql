CREATE TABLE foods (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  mass REAL NOT NULL,
  nutrition REAL[] NOT NULL,
  g2l_density REAL,
  img VARCHAR(255),
  brand VARCHAR(255),
  upc VARCHAR(255) UNIQUE
);

CREATE UNIQUE INDEX food_upc_idx ON foods(upc);

CREATE UNIQUE INDEX food_namebrand_idx ON foods(name, brand) 
  WHERE upc IS NULL;

CREATE UNIQUE INDEX food_name_idx ON foods(name) 
  WHERE brand IS NULL AND upc IS NULL;

CREATE TABLE servings (
  id SERIAL PRIMARY KEY,
  food_id INT REFERENCES foods(id) NOT NULL,
  name VARCHAR(255) NOT NULL,
  mass REAL NOT NULL
);

CREATE TABLE meals (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255),
  time TIMESTAMP WITH TIME ZONE NOT NULL,
  servings REAL NOT NULL,
  servings_consumed REAL NOT NULL,
  food_ids INT[] NOT NULL,
  food_masses REAL[] NOT NULL
);
