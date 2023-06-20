// @generated automatically by Diesel CLI.

diesel::table! {
    circuit (id) {
        id -> Integer,
        circuitRef -> Text,
        name -> Text,
        location -> Nullable<Text>,
        country -> Nullable<Text>,
        lat -> Nullable<Float>,
        lng -> Nullable<Float>,
        alt -> Nullable<Integer>,
        url -> Text,
    }
}

diesel::table! {
    constructor (id) {
        id -> Integer,
        constructorRef -> Text,
        name -> Text,
        nationality -> Nullable<Text>,
        url -> Text,
    }
}

diesel::table! {
    constructor_result (id) {
        id -> Integer,
        raceId -> Integer,
        constructorId -> Integer,
        points -> Nullable<Float>,
        status -> Nullable<Text>,
    }
}

diesel::table! {
    constructor_standing (id) {
        id -> Integer,
        raceId -> Integer,
        constructorId -> Integer,
        points -> Float,
        position -> Nullable<Integer>,
        positionText -> Nullable<Text>,
        wins -> Integer,
    }
}

diesel::table! {
    driver (id) {
        id -> Integer,
        driverRef -> Text,
        number -> Nullable<Integer>,
        code -> Nullable<Text>,
        forename -> Text,
        surname -> Text,
        dob -> Nullable<Text>,
        nationality -> Nullable<Text>,
        url -> Text,
    }
}

diesel::table! {
    driver_standing (id) {
        id -> Integer,
        raceId -> Integer,
        driverId -> Integer,
        points -> Float,
        position -> Nullable<Integer>,
        positionText -> Nullable<Text>,
        wins -> Integer,
    }
}

diesel::table! {
    lap_times (raceId, driverId, lap) {
        raceId -> Integer,
        driverId -> Integer,
        lap -> Integer,
        position -> Nullable<Integer>,
        time -> Nullable<Text>,
        milliseconds -> Nullable<Integer>,
    }
}

diesel::table! {
    pit_stops (raceId, driverId, stop) {
        raceId -> Integer,
        driverId -> Integer,
        stop -> Integer,
        lap -> Integer,
        time -> Text,
        duration -> Nullable<Text>,
        milliseconds -> Nullable<Integer>,
    }
}

diesel::table! {
    qualifying (id) {
        id -> Integer,
        raceId -> Integer,
        driverId -> Integer,
        constructorId -> Integer,
        number -> Integer,
        position -> Nullable<Integer>,
        q1 -> Nullable<Text>,
        q2 -> Nullable<Text>,
        q3 -> Nullable<Text>,
    }
}

diesel::table! {
    race (id) {
        id -> Integer,
        year -> Integer,
        round -> Integer,
        circuitID -> Integer,
        name -> Text,
        date -> Text,
        time -> Nullable<Text>,
        url -> Nullable<Text>,
        fp1_date -> Nullable<Text>,
        fp1_time -> Nullable<Text>,
        fp2_date -> Nullable<Text>,
        fp2_time -> Nullable<Text>,
        fp3_date -> Nullable<Text>,
        fp3_time -> Nullable<Text>,
        quali_date -> Nullable<Text>,
        quali_time -> Nullable<Text>,
        sprint_date -> Nullable<Text>,
        sprint_time -> Nullable<Text>,
    }
}

diesel::table! {
    result (id) {
        id -> Integer,
        raceId -> Integer,
        driverId -> Integer,
        constructorId -> Integer,
        number -> Nullable<Integer>,
        grid -> Integer,
        position -> Nullable<Integer>,
        positionText -> Text,
        positionOrder -> Integer,
        points -> Float,
        laps -> Integer,
        time -> Nullable<Text>,
        milliseconds -> Nullable<Integer>,
        fastestLap -> Nullable<Integer>,
        rank -> Nullable<Integer>,
        fastestLapTime -> Nullable<Text>,
        fastestLapSpeed -> Nullable<Text>,
        statusId -> Integer,
    }
}

diesel::table! {
    seasons (season) {
        season -> Integer,
        url -> Nullable<Text>,
    }
}

diesel::table! {
    sprint_result (id) {
        id -> Integer,
        raceId -> Integer,
        driverId -> Integer,
        constructorId -> Integer,
        number -> Nullable<Integer>,
        grid -> Integer,
        position -> Nullable<Integer>,
        positionText -> Text,
        positionOrder -> Integer,
        points -> Float,
        laps -> Integer,
        time -> Nullable<Text>,
        milliseconds -> Nullable<Integer>,
        fastestLap -> Nullable<Integer>,
        fastestLapTime -> Nullable<Text>,
        statusId -> Integer,
    }
}

diesel::table! {
    status (id) {
        id -> Integer,
        statusDesc -> Text,
    }
}

diesel::joinable!(lap_times -> driver (driverId));
diesel::joinable!(lap_times -> race (raceId));
diesel::joinable!(pit_stops -> driver (driverId));
diesel::joinable!(pit_stops -> race (raceId));
diesel::joinable!(race -> circuit (circuitID));

diesel::allow_tables_to_appear_in_same_query!(
    circuit,
    constructor,
    constructor_result,
    constructor_standing,
    driver,
    driver_standing,
    lap_times,
    pit_stops,
    qualifying,
    race,
    result,
    seasons,
    sprint_result,
    status,
);
