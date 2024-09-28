use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};
use rocket_db_pools::Database;
// use shared::{PRv2, Repo, StreakUserData, TimePeriod, TimePeriodString, UserPeriodData};
// use sqlx::{PgPool, Postgres, Transaction};

#[derive(Database, Clone, Debug)]
#[database("race-of-sloths")]
pub struct DB(PgPool);

pub mod types;

use types::{LeaderboardRecord, Statistics};

use self::types::{
    RepoLeaderboardRecord, RepoRecord, StreakRecord, User, UserCachedMetadata,
    UserContributionRecord, UserPeriodRecord, UserRecord,
};

impl DB {}
