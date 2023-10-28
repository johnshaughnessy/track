CREATE SEQUENCE global_id_seq;

CREATE OR REPLACE FUNCTION next_id(OUT result bigint) AS $$
DECLARE
  our_epoch bigint := 1505706041000;
  seq_id bigint;
  now_millis bigint;
  shard_id int := 0;
BEGIN
  SELECT nextval('global_id_seq') % 1024 INTO seq_id;
  SELECT FLOOR(EXTRACT(EPOCH FROM clock_timestamp()) * 1000) INTO now_millis;
  result := (now_millis - our_epoch) << 23;
  result := result | (shard_id << 10);
  result := result | (seq_id);
END;
$$ LANGUAGE PLPGSQL;
