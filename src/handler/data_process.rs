use std::collections::HashMap;
use chrono::Duration;
use chrono::prelude::*;
use crate::db::entities::{post_data, relay_events};
use sea_orm::{QueryFilter};
use sea_orm::entity::*;
use crate::db::connection::get_db_conn;
use std::num::ParseIntError;
use std::error::Error;
use crate::error::BoxedError;
pub async fn process_check_in_events() -> Result<i32,BoxedError> {
    let db_conn = get_db_conn().await;
    let now = Utc::now();
    let current_date = now.date_naive();
    let start_of_day = current_date.and_hms_opt(0, 0, 0);
    let end_of_day = current_date.and_hms_opt(23, 59, 59);

    let check_in_data = post_data::Entity::find()
        .filter(post_data::Column::Timestamp.gte(start_of_day))
        .filter(post_data::Column::Timestamp.lte(end_of_day))
        .filter(post_data::Column::EventType.eq("CHECK-IN"))
        .all(db_conn)
        .await?;

    let mut user_check_in_events: HashMap<String, relay_events::ActiveModel> = HashMap::new();

    for data in check_in_data {
        user_check_in_events.entry(data.address.clone()).or_insert(
            relay_events::ActiveModel {
                id: NotSet,
                event_type: ActiveValue::Set("CHECK-IN".parse().unwrap()),
                time_stamp: ActiveValue::Set(data.timestamp),
                address: ActiveValue::Set(data.address.clone()),
                project_name: ActiveValue::Set(data.project),
                sign: ActiveValue::Set(data.sign),
                event_date: ActiveValue::Set(current_date),
                duration: ActiveValue::Set(Option::from(0)),
            }
        );
    }

    for (_, event) in user_check_in_events {
        let exists = relay_events::Entity::find()
            .filter(relay_events::Column::EventType.eq("CHECK-IN"))
            .filter(relay_events::Column::Address.eq(event.address.clone().unwrap()))
            .filter(relay_events::Column::TimeStamp.eq(event.time_stamp.clone().unwrap()))
            .one(db_conn)
            .await?
            .is_some();

        if !exists {
            event.clone().insert(db_conn).await.expect("Fail to Insert Relay Event Check In");
            println!("Inserted new event CHECK-IN : {:?} {:?}", event.address, event.time_stamp);
        }
    }

    Ok(0)
}

pub async fn process_online_time_events() -> Result<i32, BoxedError> {
    let valid_interval = std::env::var("VALID_TIMESTAMP_INTERVAL").expect("VALID_TIMESTAMP_INTERVAL not set");
    let online_max_duration = std::env::var("ONLINE_TIME_MAX_DURATION").expect("ONLINE_TIME_MAX_DURATION not set");
    let online_min_duration = std::env::var("ONLINE_TIME_MIN_DURATION").expect("ONLINE_TIME_MIN_DURATION not set");

    let valid_interval = parse_duration_from_seconds_str(valid_interval.clone()).unwrap();
    let online_max_duration = parse_duration_from_seconds_str(online_max_duration.clone()).unwrap();
    let online_min_duration = parse_duration_from_seconds_str(online_min_duration.clone()).unwrap();

    let db_conn = get_db_conn().await;
    let now = Utc::now();
    let current_date = now.date_naive();
    let start_of_day = current_date.and_hms_opt(0, 0, 0);
    let end_of_day = current_date.and_hms_opt(23, 59, 59);

    let mut user_online_task_data: HashMap<String, Vec<post_data::Model>> = HashMap::new();

    let mut online_time_data = post_data::Entity::find()
        .filter(post_data::Column::Timestamp.gte(start_of_day))
        .filter(post_data::Column::Timestamp.lte(end_of_day))
        .filter(post_data::Column::EventType.eq("ONLINE-TIME"))
        .all(db_conn)
        .await?;
    online_time_data.sort_by_key(|event| event.timestamp);

    for data in online_time_data {
        let store_data = user_online_task_data.entry(data.address.clone()).or_insert(Vec::new());
        store_data.push(data);
    }

    for (address, data) in user_online_task_data {
        let valid_data = get_continue_timestamps_data(data, valid_interval);
        for data in valid_data {
            if if_valid_online_task(data.clone(), online_max_duration, online_min_duration) {
                let exists = relay_events::Entity::find()
                    .filter(relay_events::Column::EventType.eq("ONLINE-TIME"))
                    .filter(relay_events::Column::Address.eq(address.clone()))
                    .filter(relay_events::Column::TimeStamp.eq(data[0].timestamp.clone()))
                    .one(db_conn)
                    .await?
                    .is_some();

                if !exists {
                    let event = relay_events::ActiveModel {
                        id: NotSet,
                        event_type: ActiveValue::Set("ONLINE-TIME".parse().unwrap()),
                        time_stamp: ActiveValue::Set(data[0].timestamp.clone()),
                        address: ActiveValue::Set(address.clone()),
                        project_name: ActiveValue::Set(data[0].project.clone()),
                        sign: ActiveValue::Set(data[0].sign.clone()),
                        event_date: ActiveValue::Set(current_date),
                        duration: ActiveValue::Set(Option::from(duration_to_seconds(online_min_duration))),
                    };
                    event.clone().insert(db_conn).await.expect("Fail to Insert Relay Event Online Time");
                    println!("Inserted new event Online Time : {:?} {:?}", event.address, event.time_stamp);
                }

            }
        }
    }

    Ok(0)
}


fn get_continue_timestamps_data(data_list: Vec<post_data::Model>, max_interval: Duration) -> Vec<Vec<post_data::Model>> {
    if data_list.is_empty() {
        return vec![];
    }

    let mut result = Vec::new();
    let mut current_group = Vec::new();
    let mut previous_timestamp = data_list[0].timestamp;

    for data in &data_list {
        if data.timestamp.signed_duration_since(previous_timestamp) > max_interval {
            result.push(current_group);
            current_group = Vec::new();
        }
        current_group.push(data.clone());
        previous_timestamp = data.timestamp.clone();
    }

    if !current_group.is_empty() {
        result.push(current_group);
    }

    result
}


fn if_valid_online_task(data_list: Vec<post_data::Model>, max_interval: Duration, min_duration: Duration) -> bool {
    if data_list.is_empty() {
        return false;
    }

    let mut current_duration = Duration::zero();
    let mut previous_timestamp = data_list[0].timestamp;

    for data in &data_list[1..] {
        let interval = data.timestamp.signed_duration_since(previous_timestamp);
        if interval <= max_interval {
            current_duration = current_duration + interval;
            if current_duration >= min_duration {
                return true;
            }
        } else {
            current_duration = Duration::zero();
        }
        previous_timestamp = data.timestamp;
    }

    false
}

fn parse_duration_from_seconds_str(seconds_str: String) -> Result<Duration, ParseIntError> {
    let seconds: i64 = seconds_str.parse()?;
    Ok(Duration::seconds(seconds))
}

fn duration_to_seconds(duration: Duration) -> i32 {
    let seconds = duration.num_seconds();
    seconds as i32
}
