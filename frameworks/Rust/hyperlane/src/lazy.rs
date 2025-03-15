use crate::*;

pub static DB: Lazy<ArcRwLock<Option<DbPoolConnection>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));
pub static UPDATE_SQL: Lazy<ArcRwLock<HashMap<Queries, String>>> =
    Lazy::new(|| arc_rwlock(HashMap::new()));
pub static UPDATE_QUERY_SQL: Lazy<ArcRwLock<HashMap<Queries, Vec<QueryRow>>>> =
    Lazy::new(|| arc_rwlock(HashMap::new()));
pub static QUERY_SQL: Lazy<ArcRwLock<HashMap<Queries, String>>> =
    Lazy::new(|| arc_rwlock(HashMap::new()));
pub static QUERY_ALL_SQL: Lazy<ArcRwLock<String>> = Lazy::new(|| arc_rwlock(String::new()));
