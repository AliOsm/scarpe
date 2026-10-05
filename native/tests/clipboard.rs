mod common;

use common::{app, create, Harness};
use serde_json::json;

#[test]
fn clipboard_round_trips_text_and_can_be_cleared_without_a_window() {
    let mut h = Harness::new();
    assert_eq!(h.value(json!({"op":"clipboard"})), json!(""));

    let text = "آدابُ العلم\nصفحة ٧٢ — \"quoted\" \\ text";
    assert_eq!(h.value(json!({"op":"clipboard", "text":text})), json!(text));
    assert_eq!(h.value(json!({"op":"clipboard"})), json!(text));
    assert_eq!(h.value(json!({"op":"clipboard", "text":""})), json!(""));
    assert_eq!(h.value(json!({"op":"clipboard"})), json!(""));
}

#[test]
fn clipboard_requests_share_the_clipboard_with_text_fields() {
    let mut h = Harness::new();
    h.feed(&app(300, 100, &[create(3, "EditLine", 2, json!({"text":""}))]));

    h.value(json!({"op":"clipboard", "text":"العلم"}));
    h.value(json!({"op":"click", "target":{"id":3}}));
    h.value(json!({"op":"key", "key":"control_v"}));
    assert_eq!(h.node(|n| n["id"] == 3)["text"], json!("العلم"));

    h.value(json!({"op":"key", "key":"control_a"}));
    h.value(json!({"op":"type", "text":"من الحقل"}));
    h.value(json!({"op":"key", "key":"control_a"}));
    h.value(json!({"op":"key", "key":"control_c"}));
    assert_eq!(h.value(json!({"op":"clipboard"})), json!("من الحقل"));
}
