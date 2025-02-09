#[test]
fn test() {
    const VOICE_CHARACTER: &[(&str, u32)] = &comptime::all_voices!();
    let _ = VOICE_CHARACTER;
}
