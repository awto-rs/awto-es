use std::sync::RwLock;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Serialize};
use thaloto::{
    aggregate::{Aggregate, TypeId},
    event_store::{AggregateEventEnvelope, EventStore},
};

use crate::Error;

#[derive(Debug, Default)]
pub struct InMemoryEventStore {
    events: RwLock<Vec<EventRecord>>,
}

#[derive(Debug)]
pub struct EventRecord {
    created_at: DateTime<Utc>,
    aggregate_type: &'static str,
    aggregate_id: String,
    sequence: usize,
    event_data: serde_json::Value,
}

#[async_trait]
impl EventStore for InMemoryEventStore {
    type Error = Error;

    async fn load_events<A>(
        &self,
        id: Option<&<A as Aggregate>::ID>,
    ) -> Result<Vec<AggregateEventEnvelope<A>>, Self::Error>
    where
        A: Aggregate,
        <A as Aggregate>::Event: DeserializeOwned,
    {
        let events_lock = self.events.read().map_err(|_| Error::RwPoison)?;
        let events = events_lock
            .iter()
            .enumerate()
            .filter(|(_index, event)| {
                event.aggregate_type == <A as TypeId>::type_id()
                    && id
                        .map(|id| event.aggregate_id == id.to_string())
                        .unwrap_or(true)
            })
            .map(|(index, event)| {
                Result::<_, Self::Error>::Ok(AggregateEventEnvelope::<A> {
                    id: index,
                    created_at: event.created_at.into(),
                    aggregate_type: event.aggregate_type.to_string(),
                    aggregate_id: event.aggregate_id.clone(),
                    sequence: event.sequence,
                    event: serde_json::from_value(event.event_data.clone())
                        .map_err(Error::DeserializeEvent)?,
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(events)
    }

    async fn load_aggregate_sequence<A>(
        &self,
        id: &<A as Aggregate>::ID,
    ) -> Result<Option<usize>, Self::Error>
    where
        A: Aggregate,
    {
        let events_lock = self.events.read().map_err(|_| Error::RwPoison)?;
        Ok(events_lock
            .iter()
            .filter_map(|event| {
                if event.aggregate_type == <A as TypeId>::type_id()
                    && event.aggregate_id == id.to_string()
                {
                    Some(event.sequence)
                } else {
                    None
                }
            })
            .max())
    }

    async fn save_events<A>(
        &self,
        id: &<A as Aggregate>::ID,
        events: &[<A as Aggregate>::Event],
    ) -> Result<Vec<usize>, Self::Error>
    where
        A: Aggregate,
        <A as Aggregate>::Event: Serialize,
    {
        if events.is_empty() {
            return Ok(vec![]);
        }

        let sequence = self.load_aggregate_sequence::<A>(id).await?;
        let mut event_ids = Vec::with_capacity(events.len());

        let mut events_lock = self.events.write().map_err(|_| Error::RwPoison)?;
        for (index, event) in events.iter().enumerate() {
            let created_at = Utc::now();
            let aggregate_type = <A as TypeId>::type_id();
            let aggregate_id = id.to_string();
            let sequence = sequence.map(|sequence| sequence + index + 1).unwrap_or(0);
            let event_data = serde_json::to_value(event).map_err(Error::SerializeEvent)?;

            events_lock.push(EventRecord {
                created_at,
                aggregate_type,
                aggregate_id,
                sequence,
                event_data,
            });
            event_ids.push(events_lock.len());
        }

        Ok(event_ids)
    }
}
