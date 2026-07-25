-- Add migration script here
CREATE TABLE IF NOT EXISTS paste (
  id Text Primary key,
  title Varchar(100) NOT NULL,
  content Text NOT NULL,
  language Varchar(15) NOT NULL,
  views Int NOT NULL DEFAULT 0,
  one_time Bool NOT NULL DEFAULT FALSE,
  created_at Timestamp DEFAULT CURRENT_TIMESTAMP,
  expires_at Timestamp Not NULL
);
