// Copyright @ 2024 - present, R3E Network
// All Rights Reserved.

use epicchain_contract::{
    env,
    runtime::{self, log, notify},
    storage::{self, get_context, put, get},
    types::{
        builtin::{string::ByteString, array::Array},
        Any,
    },
};

mod mock_env;

#[test]
fn test_syscall_runtime_log() {
    // Setup the mock environment
    mock_env::setup();

    // Call the runtime log function
    let message = "Hello, Neo!";
    log(message);

    // Verify the syscall was made with the correct parameters
    let syscalls = mock_env::get_syscalls();
    assert_eq!(syscalls.len(), 1);
    assert_eq!(syscalls[0].name, "System.Runtime.Log");
    assert_eq!(syscalls[0].args.len(), 1);
    assert_eq!(syscalls[0].args[0], message);
}

#[test]
fn test_syscall_runtime_notify() {
    // Setup the mock environment
    mock_env::setup();

    // Call the runtime notify function
    let event_name = "TestEvent";
    let data = Array::new();
    notify(event_name, data);

    // Verify the syscall was made with the correct parameters
    let syscalls = mock_env::get_syscalls();
    assert_eq!(syscalls.len(), 1);
    assert_eq!(syscalls[0].name, "System.Runtime.Notify");
    assert_eq!(syscalls[0].args.len(), 2);
    assert_eq!(syscalls[0].args[0], event_name);
}

#[test]
fn test_syscall_storage_operations() {
    // Setup the mock environment
    mock_env::setup();

    // Get storage context
    let context = get_context();

    // Put a value in storage
    let key = "test_key";
    let value = "test_value";
    put(&context, key, value);

    // Verify the put syscall was made with the correct parameters
    let syscalls = mock_env::get_syscalls();
    assert_eq!(syscalls.len(), 2); // get_context + put
    assert_eq!(syscalls[0].name, "System.Storage.GetContext");
    assert_eq!(syscalls[1].name, "System.Storage.Put");
    assert_eq!(syscalls[1].args.len(), 3);
    assert_eq!(syscalls[1].args[1], key);
    assert_eq!(syscalls[1].args[2], value);

    // Reset syscalls
    mock_env::reset_syscalls();

    // Get the value from storage
    let retrieved_value = get::<ByteString>(&context, key);

    // Verify the get syscall was made with the correct parameters
    let syscalls = mock_env::get_syscalls();
    assert_eq!(syscalls.len(), 1);
    assert_eq!(syscalls[0].name, "System.Storage.Get");
    assert_eq!(syscalls[0].args.len(), 2);
    assert_eq!(syscalls[0].args[1], key);

    // Verify the retrieved value
    assert_eq!(retrieved_value, value);
}

#[test]
fn test_syscall_hash_calculation() {
    // This test verifies that the syscall hash calculation is correct
    // by checking that the mock environment correctly handles the syscalls
    // with the expected hash values.

    // Setup the mock environment
    mock_env::setup();

    // Call various syscalls
    let message = "Test message";
    log(message);
    
    let event_name = "TestEvent";
    let data = Array::new();
    notify(event_name, data);
    
    let context = get_context();
    let key = "test_key";
    let value = "test_value";
    put(&context, key, value);
    
    // Verify all syscalls were made with the correct parameters
    let syscalls = mock_env::get_syscalls();
    assert_eq!(syscalls.len(), 4); // log + notify + get_context + put
    
    // Check log syscall
    assert_eq!(syscalls[0].name, "System.Runtime.Log");
    assert_eq!(syscalls[0].args.len(), 1);
    assert_eq!(syscalls[0].args[0], message);
    
    // Check notify syscall
    assert_eq!(syscalls[1].name, "System.Runtime.Notify");
    assert_eq!(syscalls[1].args.len(), 2);
    assert_eq!(syscalls[1].args[0], event_name);
    
    // Check get_context syscall
    assert_eq!(syscalls[2].name, "System.Storage.GetContext");
    
    // Check put syscall
    assert_eq!(syscalls[3].name, "System.Storage.Put");
    assert_eq!(syscalls[3].args.len(), 3);
    assert_eq!(syscalls[3].args[1], key);
    assert_eq!(syscalls[3].args[2], value);
    
    // Verify that the mock environment correctly handles the syscalls
    // with the expected hash values by checking that the syscalls were
    // correctly recorded and their parameters were correctly passed.
    assert!(mock_env::verify_syscall_hashes());
}
