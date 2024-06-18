use crate::event_listener::{event_parser, Event, MonitorEventData, WorkspaceType};

#[test]
fn test_parsing_createworkspace() {
    let events = r#"createworkspace>>2"#;
    let parsed = event_parser(events.into()).unwrap();
    assert_eq!(
        parsed,
        vec![Event::WorkspaceAdded(WorkspaceType::Regular("2".into()))]
    )
}

#[test]
fn test_parsing_moveworkspace() {
    let events = r#"moveworkspace>>2,monitor-1"#;
    let parsed = event_parser(events.into()).unwrap();
    assert_eq!(
        parsed,
        vec![Event::WorkspaceMoved(MonitorEventData {
            monitor_name: "monitor-1".into(),
            workspace: WorkspaceType::Regular("2".into()),
        })]
    )
}
