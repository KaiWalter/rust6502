use apple1::test_harness::Apple1ConsoleHarness;

fn boot_console() -> Apple1ConsoleHarness {
    let harness = Apple1ConsoleHarness::start();
    harness.run_cycles(100_000);
    harness
}

#[test]
fn boots_into_wozmon_prompt() {
    let harness = boot_console();
    let output = harness.drain_output_string();

    assert!(
        output.contains('\\'),
        "expected Woz monitor prompt in output, got: {output:?}"
    );
}

#[test]
fn wozmon_store_instruction_writes_to_ram() {
    let harness = boot_console();
    let _ = harness.drain_output_string();

    harness.type_text("0280: AA BB\r");
    harness.run_cycles(300_000);

    assert_eq!(harness.peek_memory(0x0280), 0xAA);
    assert_eq!(harness.peek_memory(0x0281), 0xBB);
}

#[test]
fn wozmon_examine_instruction_prints_memory_line() {
    let harness = boot_console();
    let _ = harness.drain_output_string();

    harness.type_text("FF00\r");
    harness.run_cycles(250_000);

    let output = harness.drain_output_string();
    assert!(
        output.contains("FF00"),
        "expected address from examine command in output, got: {output:?}"
    );
    assert!(
        output.contains(':'),
        "expected formatted memory line from examine command, got: {output:?}"
    );
}
