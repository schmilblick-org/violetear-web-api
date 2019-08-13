-- This file should undo anything in `up.sql`
DELETE FROM profiles WHERE machine_name = 'test_engine_1' OR machine_name = 'test_engine_2' OR machine_name = 'test_engine_3';