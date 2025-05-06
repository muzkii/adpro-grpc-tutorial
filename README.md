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

2. 
