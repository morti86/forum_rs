-- Database generated with pgModeler (PostgreSQL Database Modeler).
-- pgModeler version: 1.2.0-beta
-- PostgreSQL version: 17.0
-- Project Site: pgmodeler.io
-- Model Author: ---
-- Tablespaces creation must be performed outside a multi lined SQL file. 
-- These commands were put in this file only as a convenience.
-- 
-- object: forum | type: TABLESPACE --
-- DROP TABLESPACE IF EXISTS forum CASCADE;
CREATE TABLESPACE forum
	OWNER postgres
	LOCATION 'forum';

-- ddl-end --



-- Database creation must be performed outside a multi lined SQL file. 
-- These commands were put in this file only as a convenience.
-- 
-- object: new_database | type: DATABASE --
-- DROP DATABASE IF EXISTS new_database;
CREATE DATABASE new_database
	ENCODING = 'UTF8';
-- ddl-end --


SET check_function_bodies = false;
-- ddl-end --

-- object: forum | type: SCHEMA --
-- DROP SCHEMA IF EXISTS forum CASCADE;
CREATE SCHEMA forum;
-- ddl-end --
ALTER SCHEMA forum OWNER TO postgres;
-- ddl-end --

SET search_path TO pg_catalog,public,forum;
-- ddl-end --

-- object: forum.user_role | type: TYPE --
-- DROP TYPE IF EXISTS forum.user_role CASCADE;
CREATE TYPE forum.user_role AS
ENUM ('user','mod','admin');
-- ddl-end --
ALTER TYPE forum.user_role OWNER TO postgres;
-- ddl-end --

-- object: forum.user_status | type: TYPE --
-- DROP TYPE IF EXISTS forum.user_status CASCADE;
CREATE TYPE forum.user_status AS
ENUM ('active','inactive','banned');
-- ddl-end --
ALTER TYPE forum.user_status OWNER TO postgres;
-- ddl-end --

-- object: forum.section_id_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS forum.section_id_seq CASCADE;
CREATE SEQUENCE forum.section_id_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE forum.section_id_seq OWNER TO postgres;
-- ddl-end --

-- object: forum.topic_id | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS forum.topic_id CASCADE;
CREATE SEQUENCE forum.topic_id
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE forum.topic_id OWNER TO postgres;
-- ddl-end --

-- object: forum.sections | type: TABLE --
-- DROP TABLE IF EXISTS forum.sections CASCADE;
CREATE TABLE forum.sections (
	id int8 NOT NULL DEFAULT nextval('forum.section_id_seq'::regclass),
	name varchar(100) NOT NULL,
	description varchar(255),
	CONSTRAINT id_pkc PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE forum.sections OWNER TO postgres;
-- ddl-end --

-- object: forum.threads | type: TABLE --
-- DROP TABLE IF EXISTS forum.threads CASCADE;
CREATE TABLE forum.threads (
	id int8 NOT NULL DEFAULT nextval('forum.topic_id'::regclass),
	title varchar(255) NOT NULL,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	content text NOT NULL,
	author uuid NOT NULL,
	section int8 NOT NULL,
	locked boolean NOT NULL DEFAULT false,
	sticky boolean NOT NULL DEFAULT false,
	CONSTRAINT id_pk PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE forum.threads OWNER TO postgres;
-- ddl-end --

-- object: forum.seq_s | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS forum.seq_s CASCADE;
CREATE SEQUENCE forum.seq_s
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE forum.seq_s OWNER TO postgres;
-- ddl-end --

-- object: forum.users | type: TABLE --
-- DROP TABLE IF EXISTS forum.users CASCADE;
CREATE TABLE forum.users (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	name varchar(100) NOT NULL,
	email varchar(100) NOT NULL,
	verified boolean NOT NULL DEFAULT false,
	password varchar(100) NOT NULL,
	verification_token varchar(255),
	token_expires_at timestamptz,
	role forum.user_role NOT NULL DEFAULT ‘user’,
	created_at timestamptz DEFAULT NOW(),
	updated_at timestamptz DEFAULT NOW(),
	description varchar(255),
	avatar varchar(100) DEFAULT 'default.png',
	facebook varchar(100),
	x_id varchar(100),
	banned_until timestamptz,
	last_online timestamptz,
	CONSTRAINT users_pk PRIMARY KEY (id),
	CONSTRAINT ban_until_check CHECK (banned_until > NOW()),
	CONSTRAINT name_unique UNIQUE (name),
	CONSTRAINT one_user_per_email UNIQUE (email)
);
-- ddl-end --
ALTER TABLE forum.users OWNER TO postgres;
-- ddl-end --

-- object: forum.posts | type: TABLE --
-- DROP TABLE IF EXISTS forum.posts CASCADE;
CREATE TABLE forum.posts (
	id int8 NOT NULL DEFAULT nextval('forum.section_id_seq'::regclass),
	content text NOT NULL,
	author uuid,
	topic int8 NOT NULL,
	comments int8,
	created_at timestamptz NOT NULL DEFAULT NOW(),
	modified_at timestamptz,
	likes int4 NOT NULL DEFAULT 0,
	CONSTRAINT post_pk PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE forum.posts OWNER TO postgres;
-- ddl-end --

-- object: forum.allowed_id | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS forum.allowed_id CASCADE;
CREATE SEQUENCE forum.allowed_id
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE forum.allowed_id OWNER TO postgres;
-- ddl-end --

-- object: forum.sections_allowed | type: TABLE --
-- DROP TABLE IF EXISTS forum.sections_allowed CASCADE;
CREATE TABLE forum.sections_allowed (
	id int4 NOT NULL DEFAULT nextval('forum.allowed_id'::regclass),
	section int8 NOT NULL,
	role forum.user_role NOT NULL,
	CONSTRAINT allowed_pk PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE forum.sections_allowed OWNER TO postgres;
-- ddl-end --

-- object: forum.hash_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS forum.hash_seq CASCADE;
CREATE SEQUENCE forum.hash_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE forum.hash_seq OWNER TO postgres;
-- ddl-end --

-- object: forum.hashtags | type: TABLE --
-- DROP TABLE IF EXISTS forum.hashtags CASCADE;
CREATE TABLE forum.hashtags (
	id int8 NOT NULL DEFAULT nextval('forum.hash_seq'::regclass),
	tag varchar(25) NOT NULL,
	topic int8 NOT NULL,
	CONSTRAINT hashtag_pk PRIMARY KEY (id),
	CONSTRAINT tag_unique UNIQUE (tag,topic)
);
-- ddl-end --
ALTER TABLE forum.hashtags OWNER TO postgres;
-- ddl-end --

-- object: forum.chat_post_ids | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS forum.chat_post_ids CASCADE;
CREATE SEQUENCE forum.chat_post_ids
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE forum.chat_post_ids OWNER TO postgres;
-- ddl-end --

-- object: forum.chat_posts | type: TABLE --
-- DROP TABLE IF EXISTS forum.chat_posts CASCADE;
CREATE TABLE forum.chat_posts (
	id int4 NOT NULL DEFAULT nextval('forum.chat_post_ids'::regclass),
	added timestamptz NOT NULL DEFAULT NOW(),
	author uuid NOT NULL,
	content varchar(255) NOT NULL,
	CONSTRAINT chat_pk PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE forum.chat_posts OWNER TO postgres;
-- ddl-end --

-- object: forum.user_warning | type: TABLE --
-- DROP TABLE IF EXISTS forum.user_warning CASCADE;
CREATE TABLE forum.user_warning (
	id int8 NOT NULL,
	user_id uuid NOT NULL,
	warn_time timestamptz NOT NULL DEFAULT NOW(),
	comment varchar(255),
	warned_by uuid NOT NULL,
	banned boolean NOT NULL DEFAULT false,
	CONSTRAINT warn_id_pk PRIMARY KEY (id),
	CONSTRAINT user_no_self_warn CHECK (warned_by <> user_id)
);
-- ddl-end --
ALTER TABLE forum.user_warning OWNER TO postgres;
-- ddl-end --

-- object: forum.pm_seq | type: SEQUENCE --
-- DROP SEQUENCE IF EXISTS forum.pm_seq CASCADE;
CREATE SEQUENCE forum.pm_seq
	INCREMENT BY 1
	MINVALUE 0
	MAXVALUE 2147483647
	START WITH 1
	CACHE 1
	NO CYCLE
	OWNED BY NONE;

-- ddl-end --
ALTER SEQUENCE forum.pm_seq OWNER TO postgres;
-- ddl-end --

-- object: forum.private_messages | type: TABLE --
-- DROP TABLE IF EXISTS forum.private_messages CASCADE;
CREATE TABLE forum.private_messages (
	id int8 NOT NULL DEFAULT nextval('forum.pm_seq'::regclass),
	author uuid,
	receiver uuid NOT NULL,
	content varchar(255) NOT NULL,
	CONSTRAINT pm_pk PRIMARY KEY (id),
	CONSTRAINT pm_author_recv CHECK (author <> receiver)
);
-- ddl-end --
ALTER TABLE forum.private_messages OWNER TO postgres;
-- ddl-end --

-- object: forum.delete_related_threads | type: FUNCTION --
-- DROP FUNCTION IF EXISTS forum.delete_related_threads() CASCADE;
CREATE OR REPLACE FUNCTION forum.delete_related_threads ()
	RETURNS trigger
	LANGUAGE plpgsql
	VOLATILE 
	CALLED ON NULL INPUT
	SECURITY INVOKER
	PARALLEL UNSAFE
	COST 1
	AS $$
CREATE OR REPLACE FUNCTION delete_related_threads()
RETURNS TRIGGER AS $$
BEGIN
    -- Delete rows from the "thread" table where section_id matches the deleted row
    DELETE FROM forum.threads
    WHERE section_id = OLD.section_id;
    -- Delete permissions for deleted section
    DELETE FROM forum.sections_allowed
    WHERE section = OLD.section_id;

    -- Return the deleted row (required for BEFORE DELETE triggers)
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;
$$;
-- ddl-end --
ALTER FUNCTION forum.delete_related_threads() OWNER TO postgres;
-- ddl-end --

-- object: tr_on_delete_section | type: TRIGGER --
-- DROP TRIGGER IF EXISTS tr_on_delete_section ON forum.sections CASCADE;
CREATE OR REPLACE TRIGGER tr_on_delete_section
	BEFORE DELETE 
	ON forum.sections
	FOR EACH ROW
	EXECUTE PROCEDURE forum.delete_related_threads();
-- ddl-end --

-- object: forum.delete_related_posts | type: FUNCTION --
-- DROP FUNCTION IF EXISTS forum.delete_related_posts() CASCADE;
CREATE OR REPLACE FUNCTION forum.delete_related_posts ()
	RETURNS trigger
	LANGUAGE plpgsql
	VOLATILE 
	CALLED ON NULL INPUT
	SECURITY INVOKER
	PARALLEL UNSAFE
	COST 1
	AS $$
CREATE OR REPLACE FUNCTION delete_related_posts()
RETURNS TRIGGER AS $$
BEGIN
    -- Delete rows from the "thread" table where section_id matches the deleted row
    DELETE FROM forum.posts
    WHERE topic = OLD.topic;

    -- Return the deleted row (required for BEFORE DELETE triggers)
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;
$$;
-- ddl-end --
ALTER FUNCTION forum.delete_related_posts() OWNER TO postgres;
-- ddl-end --

-- object: tr_delete_threads_posts | type: TRIGGER --
-- DROP TRIGGER IF EXISTS tr_delete_threads_posts ON forum.threads CASCADE;
CREATE OR REPLACE TRIGGER tr_delete_threads_posts
	BEFORE DELETE 
	ON forum.threads
	FOR EACH STATEMENT
	EXECUTE PROCEDURE forum.delete_related_posts();
-- ddl-end --

-- object: topics_sections | type: CONSTRAINT --
-- ALTER TABLE forum.threads DROP CONSTRAINT IF EXISTS topics_sections CASCADE;
ALTER TABLE forum.threads ADD CONSTRAINT topics_sections FOREIGN KEY (section)
REFERENCES forum.sections (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: topics_users | type: CONSTRAINT --
-- ALTER TABLE forum.threads DROP CONSTRAINT IF EXISTS topics_users CASCADE;
ALTER TABLE forum.threads ADD CONSTRAINT topics_users FOREIGN KEY (author)
REFERENCES forum.users (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: post_author | type: CONSTRAINT --
-- ALTER TABLE forum.posts DROP CONSTRAINT IF EXISTS post_author CASCADE;
ALTER TABLE forum.posts ADD CONSTRAINT post_author FOREIGN KEY (author)
REFERENCES forum.users (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: post_topic | type: CONSTRAINT --
-- ALTER TABLE forum.posts DROP CONSTRAINT IF EXISTS post_topic CASCADE;
ALTER TABLE forum.posts ADD CONSTRAINT post_topic FOREIGN KEY (topic)
REFERENCES forum.threads (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: comments_post | type: CONSTRAINT --
-- ALTER TABLE forum.posts DROP CONSTRAINT IF EXISTS comments_post CASCADE;
ALTER TABLE forum.posts ADD CONSTRAINT comments_post FOREIGN KEY (comments)
REFERENCES forum.posts (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: allowed_in_sec | type: CONSTRAINT --
-- ALTER TABLE forum.sections_allowed DROP CONSTRAINT IF EXISTS allowed_in_sec CASCADE;
ALTER TABLE forum.sections_allowed ADD CONSTRAINT allowed_in_sec FOREIGN KEY (section)
REFERENCES forum.sections (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: tag_topic | type: CONSTRAINT --
-- ALTER TABLE forum.hashtags DROP CONSTRAINT IF EXISTS tag_topic CASCADE;
ALTER TABLE forum.hashtags ADD CONSTRAINT tag_topic FOREIGN KEY (topic)
REFERENCES forum.threads (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: author_uuid | type: CONSTRAINT --
-- ALTER TABLE forum.chat_posts DROP CONSTRAINT IF EXISTS author_uuid CASCADE;
ALTER TABLE forum.chat_posts ADD CONSTRAINT author_uuid FOREIGN KEY (author)
REFERENCES forum.users (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: user_warned | type: CONSTRAINT --
-- ALTER TABLE forum.user_warning DROP CONSTRAINT IF EXISTS user_warned CASCADE;
ALTER TABLE forum.user_warning ADD CONSTRAINT user_warned FOREIGN KEY (user_id)
REFERENCES forum.users (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: warned_by_fk | type: CONSTRAINT --
-- ALTER TABLE forum.user_warning DROP CONSTRAINT IF EXISTS warned_by_fk CASCADE;
ALTER TABLE forum.user_warning ADD CONSTRAINT warned_by_fk FOREIGN KEY (warned_by)
REFERENCES forum.users (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: pm_author | type: CONSTRAINT --
-- ALTER TABLE forum.private_messages DROP CONSTRAINT IF EXISTS pm_author CASCADE;
ALTER TABLE forum.private_messages ADD CONSTRAINT pm_author FOREIGN KEY (author)
REFERENCES forum.users (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: pm_recv | type: CONSTRAINT --
-- ALTER TABLE forum.private_messages DROP CONSTRAINT IF EXISTS pm_recv CASCADE;
ALTER TABLE forum.private_messages ADD CONSTRAINT pm_recv FOREIGN KEY (receiver)
REFERENCES forum.users (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --


