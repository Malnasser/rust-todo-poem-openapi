-- Update existing NULL values to false
UPDATE todos SET completed = false WHERE completed IS NULL;

-- Alter the column to be NOT NULL
ALTER TABLE todos ALTER COLUMN completed SET NOT NULL;