# Advanced Programming Rust 
by Andriyo Averill Fahrezi, NPM of 2306172325

## GRPC-Tutorial

### Reflection

1. **What are the key differences between unary, server streaming, and bi-directional streaming RPC (Remote Procedure Call) methods, and in what scenarios would each be most suitable?**

| Type | Description | Use Case | Example |
| -- | --- | --- | --- |
| Unary | Single request and single response | Simple operations like fetching a single record or performing a calculation | `process_payment` function in `MyPaymentService` |
| Server Streaming | Single client request, multiple server responses | Scenarios like fetching a list of transactions or logs | `get_transaction_history` in `MyTransactionService` | 
| Bi-Directional | 	Multiple client requests and server responses streamed concurrently | Real-time chat applications or collaborative tools | `chat` in `MyChatService` (Chat applications in general) |

2. **What are the potential security considerations involved in implementing a gRPC service in Rust, particularly regarding authentication, authorization, and data encryption?**

    After implementing gRPC service in Rust, there are some security considerations that has been or needed to be handled. Such as:

    - Authentication : We can use TLS for secure communication and implement token-based authentication. Examples could be JWT, OAuth2, or mTLS to verify identities.
    - Authorization : We can implement a role-based access control (RBAC) or attribute-based access control (ABAC) to restrict access to specific RPC methods (endpoints).
    - Data Encryption : We ensure al communication iis encrypted using TLS. This is to avoid transmitting sensitive data in plaintext. We do this by `tonic::transport::Server::tls_config` to encrypt traffic.
    - Input Validation : We validate all incoming requests to prevent injection attacks and or malformed requests (data).
    - Logging and Monitoring : Log access attempts and monitor for suspicious activity. We can also do Rate Limiting to protect our services from DoS attacks.

3. **What are the potential challenges or issues that may arise when handling bidirectional streaming in Rust gRPC, especially in scenarios like chat applications?**

    In my opinion, there are some potential challenges in handling bidirectional streaming in Rust gRPC, specifically when talking about chat application. Some of the challenges may be but not limited to:
     - Concurrency : Concurrent message handling would need managing multiple streams simulatenously which could be quite complex. In this tutorial, we used `tokio` and `mpsc` channels to manage reads/writes.
     - Backpressure : Handling scenarios where one side (client or server) sends data faster than the other one can process. This would lead to uncontrolled flow that can overwhelm receivers.
     - Error Handling : Properly handling errors in one stream without disrupting the other. This is achieved by somewhat writing the same code that exist in both `grpc_client` and `grpc_server`.
     - Dropped Connections : When trying to execute the server with `cargo run`, if we somewhat launch the client first then launch the server, this would lead to some errors because the server isn't listening yet.

4. **What are the advantages and disadvantages of using the `tokio_stream::wrappers::ReceiverStream` for streaming responses in Rust gRPC services?**
   
     a. Advantages

    -  Easily bridges `tokio:mpsc` with gRPC streams.
    -  Async-friendly and integrates well with `tonic`.
    -  Provides a clean abstraction for converting a `Receiver` into a stream.

    b. Disadvantages

    - Fixed buffer size that can lead to drops or delays.
    - Error handling must be implemented manually with checking whether if it's `Ok` or not.
    - Requires explicit spawn and channel management.
    - Limited flexibility compared to custom stream impementations.

5. **In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity, promoting maintainability and extensibility over time?**

      To improve code structure and promote reuse and modularity, the application can be organized by splitting each service—such as `PaymentService`, `TransactionService`, and `ChatService`—into separate modules, with their core logic encapsulated in distinct service layers (e.g., `payment_service`, `transaction_service`). Common utilities like authentication, logging, error handling, and validation can be extracted into shared helper modules to avoid duplication. Protocol Buffers should be defined in a shared `.proto` file to maintain consistency across services. Additionally, configuration settings such as server addresses and TLS options should be centralized for easier management. To further enhance modularity and testability, dependencies like database connections can be injected into service constructors, allowing service logic to remain decoupled from implementation details.

6. **In the `MyPaymentService` implementation, what additional steps might be necessary to
handle more complex payment processing logic?**

    To handle more complex payment processing logic, we could do several things such as validation, error handling, and maybe integrate it with external payment gateway such as GoPay or QRIS. For more detailed, it could be:

     -  Validation : Validate payment details, could be for checking whether the user's balance is sufficient or not, whether it is the correct user id or not, or whether if it's a fraud or not.
     -  Database Integration : Store transactions and audit logs with payment records in a database and not just a variable.
     -  External Payment Gateways : Integrate API with GoPay, QRIS, DANA, OVO, or any other EPS.
     -  Retry Logic : An option to retry whether there is a downstream service failures

7. **What impact does the adoption of gRPC as a communication protocol have on the overall architecture and design of distributed systems, particularly in terms of interoperability with other technologies and platforms?**

     There are some positive and negative impacts of adpoting gRPC as a communcation tool to our architecture or system:

     a. Pros

    -  Language-agnostic via Protobuf.
    -  Interoperability: gRPC supports multiple languages, making it easier to integrate with diverse systems.
    -  Built-in streaming support.
    -  Performance: Efficient serialization with Protocol Buffers and HTTP/2 reduces latency.
    -  Scalability: Streaming capabilities enable real-time communication, improving scalability.

    b. Cons

    - Requires Protobuf tooling and gRPC libraries.
    - Harder to debug (compared to JSON/REST).
    - May need adapters for legacy systems or public APIs (Design Pattern).
    - Requires all systems to support Protocol Buffers and HTTP/2.
      
8. **What are the advantages and disadvantages of using HTTP/2, the underlying protocol for gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?**

| Protocol | Pros | Cons | 
| -- | --- | --- |
| HTTP/2 | Multiplexing (multiple streams over a single connection reduce latency), header compression that reduces bandwidth usage, and enables real-time communication | Not universally supported on all networks and more complex than HTTP/1.1 |
| HTTP/1.1 | 	Broad support | No native streaming/multiplexing | 
| WebSockets | 	Real-time bidirectional support | Harder to manage at scale (stateful)| 

9.  **How does the request-response model of REST APIs contrast with the bidirectional streaming capabilities of gRPC in terms of real-time communication and responsiveness?**

    Well, request-reponse model, aka REST (one request = one response) APIs is used best for CRUD operations. It is used basically as to limited real-time capabilities (e.g., polling or WebSockets). Meanwhile, gRPC enables real time, event driven communication (supports bidirectional streaming). By this, bi-directional streaming, it could outperform REST for chat or live dashboard. It is also proven to be more efficient for high-frequency, low-latency interactions.

10. **What are the implications of the schema-based approach of gRPC, using Protocol Buffers, compared to the more flexible, schema-less nature of JSON in REST API payloads?**


| Aspect | Protobuf (gRPC) | JSON (REST) | 
| -- | --- | --- |
| Schema | 	Strongly typed, defined via `.proto` | Flexible, but error-prone without schema | 
| Size | Smaller (binary) | Larger (text-based) |
| Speed | Faster (compiled, binary) | Slower (parsing text) |
| Evolution | Supports versioning via tags | Breaking changes can occur | 
| Human-Readable| No | Yes | 
