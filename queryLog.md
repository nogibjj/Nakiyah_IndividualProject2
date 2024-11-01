```sql
CREATE TABLE worker_health (
            Employee_ID INTEGER PRIMARY KEY,
            Age INTEGER,
            Job_Role TEXT,
            Industry TEXT,
            Years_of_Experience INTEGER,
            Work_Location TEXT,
            Hours_Worked_Per_Week INTEGER,
            Mental_Health_Condition TEXT,
            Access_to_Mental_Health_Resources BOOLEAN
        );
```

```sql
SELECT * FROM worker_health LIMIT 20;
```

```sql
SELECT * FROM worker_health WHERE Employee_ID = 101;
```

```sql
INSERT INTO worker_health VALUES (101, 28, 'Data Scientist', 'Tech', 5, 'Remote', 40, 'None', true);
```

```sql
SELECT * FROM worker_health WHERE Employee_ID = 101;
```

```sql
UPDATE worker_health SET 
         Age = 30, Job_Role = 'Senior Data Scientist', Industry = 'Tech', 
         Years_of_Experience = 6, Work_Location = 'On-site', 
         Hours_Worked_Per_Week = 45, Mental_Health_Condition = 'None', 
         Access_to_Mental_Health_Resources = false 
         WHERE Employee_ID = 101;
```

```sql
DELETE FROM worker_health WHERE Employee_ID = 101;
```

```sql
SELECT * FROM worker_health LIMIT 20;
```

```sql
INSERT INTO worker_health VALUES (201, 30, 'Tester', 'Tech', 5, 'Remote', 40, 'None', true);
```

```sql
SELECT * FROM worker_health WHERE Employee_ID = 201;
```

```sql
SELECT * FROM worker_health WHERE Employee_ID = 201;
```

```sql
SELECT * FROM worker_health WHERE Employee_ID = 201;
```

```sql
CREATE TABLE worker_health (
            Employee_ID INTEGER PRIMARY KEY,
            Age INTEGER,
            Job_Role TEXT,
            Industry TEXT,
            Years_of_Experience INTEGER,
            Work_Location TEXT,
            Hours_Worked_Per_Week INTEGER,
            Mental_Health_Condition TEXT,
            Access_to_Mental_Health_Resources BOOLEAN
        );
```

```sql
SELECT * FROM worker_health LIMIT 20;
```

```sql
SELECT * FROM worker_health WHERE Employee_ID = 101;
```

```sql
INSERT INTO worker_health VALUES (101, 28, 'Data Scientist', 'Tech', 5, 'Remote', 40, 'None', true);
```

```sql
SELECT * FROM worker_health WHERE Employee_ID = 101;
```

```sql
UPDATE worker_health SET 
         Age = 30, Job_Role = 'Senior Data Scientist', Industry = 'Tech', 
         Years_of_Experience = 6, Work_Location = 'On-site', 
         Hours_Worked_Per_Week = 45, Mental_Health_Condition = 'None', 
         Access_to_Mental_Health_Resources = false 
         WHERE Employee_ID = 101;
```

```sql
DELETE FROM worker_health WHERE Employee_ID = 101;
```

