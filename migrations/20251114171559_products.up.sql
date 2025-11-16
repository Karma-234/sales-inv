-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "products" (
    id UUID PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name VARCHAR(255) NOT NULL ,
    price FLOAT NOT NULL,
    quantity INT NOT NULL,
    pack_price FLOAT NULL,
    created_at TIMESTAMP 
        WITH 
            TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() 
);
