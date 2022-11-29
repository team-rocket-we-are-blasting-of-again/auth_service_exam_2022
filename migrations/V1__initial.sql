CREATE TABLE users (
    id serial4 NOT NULL,
    user_role text NOT NULL,
    role_id int4 NOT NULL,
    email text NOT NULL,
    user_password text NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)
);