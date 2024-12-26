# OpenTier_Task
Solution for OpenTier Server-Client Handling Task

# Solution

# Documentation: Bug Analysis and Fix Report

## 1. Bug Analysis

### Identified Bugs in the Original Implementation:
1. **Message Decoding and Processing:**
   - The original implementation was hardcoded to handle only `EchoMessage` types. 
   - It failed to provide extensibility for additional message types (e.g., `AddRequest`).

2. **Client Handling:**
   - The client processing logic didn't support continuous interactions in a loop.
   - No handling for unsupported or malformed messages was present.

3. **Error Management:**
   - Errors in message decoding were only logged but not handled gracefully, leading to potential runtime issues.

4. **Server Architecture:**
   - The original server lacked proper client connection management (e.g., maintaining a list of active clients).
   - Client disconnections weren't handled effectively, risking resource leaks.

5. **Concurrency Issues:**
   - The original server used blocking logic for client handling, which could block other clients from connecting or communicating.

---

## 2. Fix Implementation

### How Architectural Flaws Were Addressed:
1. **Enhanced Message Handling:**
   - Introduced `client_message` and `server_message` enums to represent various message types.
   - Added support for `EchoMessage` and `AddRequest` handling, with room for future extensibility.

2. **Robust Client Management:**
   - Refactored the `Client` struct to include a continuous loop for processing messages.
   - Incorporated proper handling for malformed or unsupported messages.

3. **Improved Error Handling:**
   - Decoding errors now log the issue and gracefully continue without crashing the server.

4. **Efficient Server Design:**
   - Introduced a shared, thread-safe client list using `Arc<Mutex<Vec<TcpStream>>>`.
   - Clients are tracked and removed upon disconnection.

5. **Concurrency and Non-Blocking Mode:**
   - The server operates in non-blocking mode, ensuring new connections are not stalled by existing ones.
   - Used threading for concurrent client handling.

---

## Summary of Improvements
The refactored code provides a robust and extensible foundation for handling multiple message types and managing multiple client connections concurrently. The use of non-blocking I/O and proper concurrency mechanisms ensures scalability and reliability.

## Added Test Case: test_two_clients 
A test case named test_two_clients was implemented to validate the server's ability to handle multiple clients simultaneously. The test ensures the following functionalities:

EchoMessage Handling: A client sends an EchoMessage, and the server responds with the same content.
AddRequest Handling: Another client sends an AddRequest, and the server responds with the correct sum in an AddResponse.
Client-Server Interaction: Both clients successfully connect, communicate, and disconnect from the server.
The test verifies proper message handling and ensures the server can process concurrent requests without errors. The test passed successfully, confirming the server's robustness in handling multiple clients.

## Test Results (Screenshots):
1. **test_client_connection:**

![test_client_connection](https://github.com/user-attachments/assets/cba9da31-dd5c-4d9d-a258-b82eff1229c1)


2. **test_client_echo_message:**
   
![test_client_echo_message](https://github.com/user-attachments/assets/809776b1-9d56-4211-bbbf-b12027a2d73d)

3. **test_multiple_echo_messages:**

![test_multiple_echo_messages](https://github.com/user-attachments/assets/30c277a4-2ae2-488d-bd47-2e257ceaf9d0)

4. **test_multiple_clients:**

![test_multiple_clients](https://github.com/user-attachments/assets/d7167184-67b0-44f5-86f7-b84c6c6f795c)

5. **test_client_add_request:**

![test_client_add_request](https://github.com/user-attachments/assets/5e4fcb05-6904-4aa4-96c6-5f6bde935443)


6. **test_two_clients:**

![test_two_clients](https://github.com/user-attachments/assets/3abc5ec3-05ff-4091-80d5-f51edb45d3e6)
