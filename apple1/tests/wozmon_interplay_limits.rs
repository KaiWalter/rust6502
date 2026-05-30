use apple1::test_harness::Apple1ConsoleHarness;

fn boot_console() -> Apple1ConsoleHarness {
    let harness = Apple1ConsoleHarness::start();
    harness.run_cycles(120_000);
    let _ = harness.drain_output_string();
    harness
}

fn run_command(harness: &Apple1ConsoleHarness, command: &str) -> String {
    harness.type_text(command);
    harness.run_cycles(700_000);
    harness.drain_output_string()
}

#[test]
fn store_crosses_address_bus_block_boundary() {
    let harness = boot_console();

    let _ = run_command(&harness, "00FE: 11 22 33 44\r");

    assert_eq!(harness.peek_memory(0x00FE), 0x11);
    assert_eq!(harness.peek_memory(0x00FF), 0x22);
    assert_eq!(harness.peek_memory(0x0100), 0x33);
    assert_eq!(harness.peek_memory(0x0101), 0x44);
}

#[test]
fn store_reaches_upper_ram_limit() {
    let harness = boot_console();

    let _ = run_command(&harness, "0FFC: A1 A2 A3 A4\r");

    assert_eq!(harness.peek_memory(0x0FFC), 0xA1);
    assert_eq!(harness.peek_memory(0x0FFD), 0xA2);
    assert_eq!(harness.peek_memory(0x0FFE), 0xA3);
    assert_eq!(harness.peek_memory(0x0FFF), 0xA4);
}

#[test]
fn long_store_sequence_keeps_cpu_pia_handshake_stable() {
    let harness = boot_console();

    let _ = run_command(
        &harness,
        "0200: 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F\r",
    );

    for (offset, expected) in (0x00u8..=0x0F).enumerate() {
        assert_eq!(harness.peek_memory(0x0200 + offset as u16), expected);
    }
}

#[test]
fn lowercase_keyboard_input_is_uppercased_through_pia_path() {
    let harness = boot_console();

    let output = run_command(&harness, "ab\r");

    assert!(
        output.contains("AB"),
        "expected uppercase echo from keyboard path, got: {output:?}"
    );
}

#[test]
fn newline_input_is_translated_and_accepted_by_monitor() {
    let harness = boot_console();

    let _ = run_command(&harness, "0288: 5A\n");

    assert_eq!(harness.peek_memory(0x0288), 0x5A);
}

#[test]
fn writing_to_pia_display_register_produces_terminal_output() {
    let harness = boot_console();

    let output = run_command(&harness, "D012: C8\r");

    assert!(
        output.contains('H'),
        "expected display output from PIA-mapped write, got: {output:?}"
    );
}
