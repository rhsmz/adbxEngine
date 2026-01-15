#[cfg(test)]
mod tests {
    use adbx_editor::communication::{EditorRuntimeCommunication, ConnectionState};
    use adbx_shared::EditorMessage;
    use std::time::Instant;

    #[test]
    fn test_communication_resource_default() {
        let comm = EditorRuntimeCommunication::default();
        assert_eq!(comm.connection_state, ConnectionState::Disconnected);
        assert!(comm.tcp_stream.is_none());
        assert!(comm.tcp_port.is_none());
        assert_eq!(comm.connection_attempts, 0);
        assert!(comm.last_connection_attempt.is_none());
        assert!(comm.next_retry_at.is_none());
    }

    #[test]
    fn test_connection_state_transitions() {
        let mut comm = EditorRuntimeCommunication::default();

        // Initial state
        assert_eq!(comm.connection_state, ConnectionState::Disconnected);

        // Connecting state
        comm.connection_state = ConnectionState::Connecting;
        comm.last_connection_attempt = Some(Instant::now());
        comm.connection_attempts = 1;
        assert_eq!(comm.connection_state, ConnectionState::Connecting);
        assert_eq!(comm.connection_attempts, 1);

        // Connected state
        comm.connection_state = ConnectionState::Connected;
        assert_eq!(comm.connection_state, ConnectionState::Connected);

        // Reconnecting state
        comm.connection_state = ConnectionState::Reconnecting;
        assert_eq!(comm.connection_state, ConnectionState::Reconnecting);
    }

    #[test]
    fn test_message_serialization() {
        // EditorMessageのシリアライズ/デシリアライズをテスト
        let message = EditorMessage::SelectEntity { entity_id: 42 };

        let json = serde_json::to_string(&message).expect("Serialization failed");
        let deserialized: EditorMessage = serde_json::from_str(&json).expect("Deserialization failed");

        match deserialized {
            EditorMessage::SelectEntity { entity_id } => assert_eq!(entity_id, 42),
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_send_to_runtime_without_connection() {
        let comm = EditorRuntimeCommunication::default();
        let message = EditorMessage::SelectEntity { entity_id: 1 };

        // 接続されていない状態での送信はエラーを返すはず
        let result = adbx_editor::communication::message_sending::send_to_runtime(&comm, message);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not connected"));
    }

    #[test]
    fn test_receive_from_runtime_without_connection() {
        let mut comm = EditorRuntimeCommunication::default();

        // 接続されていない状態での受信は空のメッセージリストを返すはず
        let messages = adbx_editor::communication::message_receiving::receive_from_runtime(&mut comm);
        assert!(messages.is_empty());
    }
}
