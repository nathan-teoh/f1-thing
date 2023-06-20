use axum::Json;
use diesel::{prelude::*};
use serde::{Deserialize, Serialize};

use crate::schema::*;
#[derive(Queryable, Selectable)]
#[diesel(table_name=circuit)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Circuit{
    pub id: i32,
    pub circuitRef: String,
    pub name: String,
    pub location: Option<String>,
    pub country: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub alt: Option<i32>,
    pub url: String,
}


#[derive(Queryable, Selectable,Identifiable)]
#[diesel(table_name=constructor)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Constructor{
    pub id: i32,
    pub constructorRef: String,
    pub name: String,
    pub nationality: Option<String>,
    pub url: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name=constructor_result)]
#[diesel(belongs_to(Constructors))]
#[diesel(belongs_to(Races))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ConstructorResult{
    pub id: i32,
    pub raceId: i32,
    pub constructorId: i32,
    pub points: Option<f32>,
    pub status: Option<String>,
}

#[derive(Queryable, Selectable,Identifiable)]
#[diesel(table_name=constructor_standing)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Race))]
#[diesel(belongs_to(Constructor))]
pub struct ConstructorStanding{
    pub id: i32,
    pub raceId: i32,
    pub constructorId: i32,
    pub points: f32,
    pub position: Option<i32>,
    pub positionText: Option<String>,
    pub wins: i32
}

#[derive(Queryable, Selectable,Identifiable)]
#[diesel(table_name=driver)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Driver{
    pub id: i32,
    pub driverRef: String,
    pub number: Option<i32>,
    pub code: Option<String>,
    pub forename: String,
    pub surname: String,
    pub dob: Option<String>,
    pub nationality: Option<String>,
    pub url: String
}

#[derive(Queryable, Selectable,Identifiable)]
#[diesel(table_name=driver_standing)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Race))]
#[diesel(belongs_to(Driver))]
pub struct DriverStanding{
    pub id: i32,
    pub raceId: i32,
    pub driverId: i32,
    pub points: f32,
    pub position: Option<i32>,
    pub positionText: Option<String>,
    pub wins: i32
}

#[derive(Queryable, Selectable)]
#[diesel(table_name=lap_times)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Race,foreign_key=raceId))]
#[diesel(belongs_to(Driver,foreign_key=driverId))]
pub struct LapTimes{
    pub raceId: i32,
    pub driverId: i32,
    pub lap: i32,
    pub position: Option<i32>,
    pub time: Option<String>,
    pub milliseconds: Option<i32>
}

#[derive(Queryable, Selectable)]
#[diesel(table_name=pit_stops)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Race,foreign_key=raceId))]
#[diesel(belongs_to(Driver,foreign_key=driverId))]
pub struct PitStops{
    pub raceId: i32,
    pub driverId: i32,
    pub stop: i32,
    pub lap: i32,
    pub time: String,
    pub duration: Option<String>,
    pub milliseconds: Option<i32>
}

#[derive(Queryable, Selectable,Identifiable)]
#[diesel(table_name=qualifying)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Race,foreign_key=raceId))]
#[diesel(belongs_to(Driver,foreign_key=driverId))]
#[diesel(belongs_to(Constructor,foreign_key=constructorId))]
pub struct Qualifying{
    pub id: i32,
    pub raceId: i32,
    pub driverId: i32,
    pub constructorId: i32,
    pub number: i32,
    pub position: Option<i32>,
    pub q1: Option<String>,
    pub q2: Option<String>,
    pub q3: Option<String>
}

#[derive(Queryable, Selectable,Identifiable)]
#[diesel(table_name=race)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Seasons,foreign_key=year))]
#[diesel(belongs_to(Circuit,foreign_key=circuitID))]
pub struct Races{
    pub id: i32,
    pub year: i32,
    pub round: i32,
    pub circuitID: i32,
    pub name: String,
    pub date: String,
    pub time: Option<String>,
    pub url: Option<String>,
    pub fp1_date: Option<String>,
    pub fp1_time: Option<String>,
    pub fp2_date: Option<String>,
    pub fp2_time: Option<String>,
    pub fp3_date: Option<String>,
    pub fp3_time: Option<String>,
    pub quali_date: Option<String>,
    pub quali_time: Option<String>,
    pub sprint_date: Option<String>,
    pub sprint_time: Option<String>,
}

#[derive(Queryable, Selectable,Identifiable)]
#[diesel(table_name=result)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Race,foreign_key=raceId))]
#[diesel(belongs_to(Driver,foreign_key=driverId))]
#[diesel(belongs_to(Constructor,foreign_key=constructorId))]
#[diesel(belongs_to(Status,foreign_key=statusId))]

pub struct Result{
    pub id: i32,
    pub raceId: i32,
    pub driverId: i32,
    pub constructorId: i32,
    pub number: Option<i32>,
    pub grid: i32,
    pub position: Option<i32>,
    pub positionText: String,
    pub positionOrder: i32,
    pub points: f32,
    pub laps: i32,
    pub time: Option<String>,
    pub milliseconds: Option<i32>,
    pub fastestLap: Option<i32>,
    pub rank: Option<i32>,
    pub fastestLapTime: Option<String>,
    pub fastestLapSpeed: Option<String>,
    pub statusId: i32
}

#[derive(Queryable, Selectable,Identifiable)]
#[diesel(table_name=sprint_result)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Race,foreign_key=raceId))]
#[diesel(belongs_to(Driver,foreign_key=driverId))]
#[diesel(belongs_to(Constructor,foreign_key=constructorId))]
#[diesel(belongs_to(Status,foreign_key=statusId))]

pub struct SprintResult{
    pub id: i32,
    pub raceId: i32,
    pub driverId: i32,
    pub constructorId: i32,
    pub number: Option<i32>,
    pub grid: i32,
    pub position: Option<i32>,
    pub positionText: String,
    pub positionOrder: i32,
    pub points: f32,
    pub laps: i32,
    pub time: Option<String>,
    pub milliseconds: Option<i32>,
    pub fastestLap: Option<i32>,
    pub fastestLapTime: Option<String>,
    pub statusId: i32
}


#[derive(Queryable, Selectable, Deserialize,Debug,Serialize)]
#[diesel(table_name=seasons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct Seasons{
    pub season: i32,
    pub url: Option<String>
}

#[derive(Queryable, Selectable,Identifiable)]
#[diesel(table_name=status)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct Status{
    pub id: i32,
    pub statusDesc: String
}




