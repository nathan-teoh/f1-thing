-- Your SQL goes here


CREATE TABLE IF NOT EXISTS circuit(
    id INTEGER PRIMARY KEY NOT NULL,
    circuitRef TEXT NOT NULL,
    name TEXT NOT NULL,
    location TEXT,
    country TEXT,
    lat REAL,
    lng REAL,
    alt INTEGER,
    url TEXT NOT NULL UNIQUE

);

CREATE TABLE IF NOT EXISTS constructor(
    id INTEGER PRIMARY KEY NOT NULL,
    constructorRef TEXT NOT NULL,
    name TEXT NOT NULL UNIQUE,
    nationality TEXT,
    url TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS constructor_result(
    id INTEGER PRIMARY KEY NOT NULL,
    raceId INTEGER NOT NULL,
    constructorId INTEGER NOT NULL,
    points REAL,
    status TEXT,
    FOREIGN KEY(raceId) REFERENCES race(raceId),
    FOREIGN KEY(constructorId) REFERENCES constructor(constructorId)
);

CREATE TABLE IF NOT EXISTS constructor_standing(
    id INTEGER PRIMARY KEY NOT NULL,
    raceId INTEGER NOT NULL,
    constructorId INTEGER NOT NULL,
    points REAL NOT NULL,
    position INTEGER,
    positionText TEXT,
    wins INTEGER NOT NULL,
    FOREIGN KEY(raceId) REFERENCES race(raceId),
    FOREIGN KEY(constructorId) REFERENCES constructor(constructorId)
);

CREATE TABLE IF NOT EXISTS driver(
    id INTEGER PRIMARY KEY NOT NULL,
    driverRef TEXT NOT NULL,
    number INTEGER,
    code TEXT,
    forename TEXT NOT NULL,
    surname TEXT NOT NULL,
    dob TEXT,
    nationality TEXT,
    url TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS driver_standing(
    id INTEGER PRIMARY KEY NOT NULL,
    raceId INTEGER NOT NULL,
    driverId INTEGER NOT NULL,
    points REAL NOT NULL,
    position INT,
    positionText TEXT,
    wins INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS lap_times(
    raceId INTEGER NOT NULL,
    driverId INTEGER NOT NULL,
    lap INTEGER NOT NULL,
    position INTEGER,
    time TEXT,
    milliseconds INTEGER,
    PRIMARY KEY(raceId,driverId,lap),
    FOREIGN KEY(raceId) REFERENCES race(id),
    FOREIGN KEY(driverId) REFERENCES driver(id)
);

CREATE TABLE IF NOT EXISTS pit_stops(
    raceId INTEGER NOT NULL,
    driverId INTEGER NOT NULL,
    stop INTEGER NOT NULL,
    lap INTEGER NOT NULL,
    time TEXT NOT NULL,
    duration TEXT,
    milliseconds INTEGER,
    PRIMARY KEY(raceId,driverId,stop),
    FOREIGN KEY(raceId) REFERENCES race(id),
    FOREIGN KEY(driverId) REFERENCES driver(id)
);

CREATE TABLE IF NOT EXISTS qualifying(
    id INTEGER PRIMARY KEY NOT NULL,
    raceId INTEGER NOT NULL,
    driverId INTEGER NOT NULL,
    constructorId INTEGER NOT NULL,
    number INTEGER NOT NULL,
    position INTEGER,
    q1 TEXT,
    q2 TEXT,
    q3 TEXT,
    FOREIGN KEY(raceId) REFERENCES race(Id),
    FOREIGN KEY (constructorId) REFERENCES constructor(Id),
    FOREIGN KEY (driverId) REFERENCES driver(Id)
);


CREATE TABLE IF NOT EXISTS race(
    id INTEGER PRIMARY KEY NOT NULL,
    year INTEGER NOT NULL,
    round INTEGER NOT NULL,
    circuitID INTEGER NOT NULL,
    name TEXT NOT NULL,
    date TEXT NOT NULL,
    time TEXT,
    url TEXT UNIQUE,
    fp1_date TEXT,
    fp1_time TEXT,
    fp2_date TEXT,
    fp2_time TEXT,
    fp3_date TEXT,
    fp3_time TEXT,
    quali_date TEXT,
    quali_time TEXT,
    sprint_date TEXT,
    sprint_time TEXT,
    FOREIGN KEY(year) REFERENCES seasons(year),
    FOREIGN KEY(circuitID) REFERENCES circuit(id)
    --circuits foreign key
);


CREATE TABLE IF NOT EXISTS result(
    id INTEGER PRIMARY KEY NOT NULL,
    raceId INTEGER NOT NULL,
    driverId INTEGER NOT NULL,
    constructorId INTEGER NOT NULL,
    number INTEGER,
    grid INTEGER NOT NULL,
    position INTEGER,
    positionText TEXT NOT NULL,
    positionOrder INTEGER NOT NULL,
    points REAL NOT NULL,
    laps INTEGER NOT NULL,
    time TEXT,
    milliseconds INTEGER,
    fastestLap INTEGER,
    rank INTEGER,
    fastestLapTime TEXT,
    fastestLapSpeed TEXT,
    statusId INTEGER NOT NULL,
    FOREIGN KEY(raceId) REFERENCES race(Id),
    FOREIGN KEY(driverId) REFERENCES driver(Id),
    FOREIGN KEY(constructorId) REFERENCES constructor(Id),
    FOREIGN KEY(statusId) REFERENCES status(Id)
);

CREATE TABLE IF NOT EXISTS sprint_result(
    id INTEGER PRIMARY KEY NOT NULL,
    raceId INTEGER NOT NULL,
    driverId INTEGER NOT NULL,
    constructorId INTEGER NOT NULL,
    number INTEGER,
    grid INTEGER NOT NULL,
    position INTEGER,
    positionText TEXT NOT NULL,
    positionOrder INTEGER NOT NULL,
    points REAL NOT NULL,
    laps INTEGER NOT NULL,
    time TEXT,
    milliseconds INTEGER,
    fastestLap INTEGER,
    fastestLapTime TEXT,
    statusId INTEGER NOT NULL,
    FOREIGN KEY(statusId) REFERENCES status(Id)
);

CREATE TABLE IF NOT EXISTS seasons(
    season INTEGER NOT NULL PRIMARY KEY NOT NULL,
    url TEXT
);

CREATE TABLE IF NOT EXISTS status(
    id INTEGER PRIMARY KEY NOT NULL,
    statusDesc TEXT NOT NULL
);
