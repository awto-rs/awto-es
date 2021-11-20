use helpers::{attribute_macro, derive_macro};

mod derive_macros;
mod helpers;
mod proc_macros;

derive_macro!(Identity, identity);
derive_macro!(Command, aggregate);
derive_macro!(Event, aggregate);
derive_macro!(StreamTopic);
derive_macro!(CommandMessage);
derive_macro!(AggregateType);
derive_macro!(PgRepository);
derive_macro!(CombinedEvent);

attribute_macro!(aggregate_events, AggregateEvents);
attribute_macro!(aggregate_commands, AggregateCommands);
