use std::{
    borrow::Borrow,
    io::{Read, Write},
    process::{Command, Stdio},
    thread,
    time::Duration,
};

// Spawn a thread to run the main function
#[test]
fn test_opening_file() {
    let main_thread_handle = thread::spawn(|| {
        // Run the main function with the desired arguments
        let mut child = Command::new("target/debug/editor")
            .arg("tests/mocks/static") // Provide the path to a test file
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute main");

        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        std::thread::spawn(move || {
            stdin
                .write_all("d".as_bytes())
                .expect("Failed to write to stdin");
            std::thread::sleep(Duration::from_secs(1));
            stdin
                .write_all("d".as_bytes())
                .expect("failed to write to stdin");
        });

        std::thread::sleep(Duration::from_secs(1));

        // I want to read the stdout at this point, while the above process is running as it is a
        // long running tui
        let mut output_string = String::default();
        let _ = child
            .stdout
            .take()
            .unwrap()
            .read_to_string(&mut output_string);

        assert!(output_string.contains("This is a static test file."));
    });
    main_thread_handle.join().unwrap();
}
