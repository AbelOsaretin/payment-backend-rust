# Payment Backend - Rust

A high-performance payment processing backend built with Rust and the Axum web framework. This application integrates with the Paystack payment gateway to handle payment initialization, verification, and webhook processing.

## 🚀 Features

- **Payment Initialization**: Initiate payments through Paystack API
- **Redirect Flow**: Support for payment flow with automatic redirects to Paystack
- **Webhook Handling**: Secure webhook endpoint with HMAC-SHA512 signature verification
- **Async/Await**: Built on Tokio for high-concurrency async operations
- **RESTful API**: Clean and intuitive API endpoints
- **Error Handling**: Proper HTTP status codes and error responses
- **Environment Configuration**: Support for environment variables via `.env` file

## 📋 Prerequisites

- Rust 1.70+ (2024 edition)
- Paystack API key (get from [Paystack Dashboard](https://dashboard.paystack.com))
- Cargo (comes with Rust)

## 🔧 Installation

### 1. Clone/Setup the Project

```bash
cd payment-backend-rust
```

### 2. Configure Environment Variables

Create a `.env` file in the project root:

```env
PAYSTACK_API_KEY=your_paystack_secret_key_here
PAYSTACK_INITIALIZE_URL=https://api.paystack.co/transaction/initialize
PAYSTACK_TRANS_VERIFY_URL=https://api.paystack.co/transaction/verify/
```

Get your API key from [Paystack Dashboard](https://dashboard.paystack.com/settings/api-keys).

### 3. Install Dependencies

Dependencies are defined in `Cargo.toml` and will be installed automatically when running:

```bash
cargo build
```

## 📦 Dependencies

- **axum** (0.8.8) - Web framework for building async APIs
- **tokio** (1.42) - Async runtime with full features
- **serde** (1.0) - Serialization/deserialization framework
- **serde_json** (1.0) - JSON support
- **reqwest** (0.12) - HTTP client with JSON support
- **chrono** (0.4) - Date/time handling
- **hmac** (0.12.1) - HMAC authentication
- **sha2** (0.10.9) - SHA-256/SHA-512 hashing
- **hex** (0.4.3) - Hex encoding/decoding
- **dotenvy** (0.15) - Environment variable loading

## 🎯 API Endpoints

### 1. Health Check

```
GET /health
```

Returns server status.

**Response:**

```
200 OK
OK 🦀
```

---

### 2. Echo Endpoint

```
POST /echo
Content-Type: text/plain

[request body]
```

Echoes back the request body.

---

### 3. Initialize Payment

```
POST /initialize
Content-Type: application/json

{
  "email": "customer@example.com",
  "amount": "50000"
}
```

Returns authorization URL and access code from Paystack.

**Response:**

```json
{
  "status": true,
  "message": "Authorization URL created",
  "data": {
    "authorization_url": "https://checkout.paystack.com/...",
    "access_code": "ACCESS_CODE",
    "reference": "REFERENCE_CODE"
  }
}
```

---

### 4. Initialize Payment with Redirect

```
POST /initialize/redirect/{email}/{amount}
```

Automatically redirects to Paystack payment page.

**Example:**

```
POST /initialize/redirect/customer@example.com/50000
```

---

### 5. Webhook

```
POST /webhook
Content-Type: application/json
X-Paystack-Signature: [signature]

[Paystack webhook payload]
```

Secure webhook endpoint that verifies Paystack signatures using HMAC-SHA512.

**Supported Events:**

- `charge.success` - Payment successful

---

## 🚀 Running the Application

### Development Mode

```bash
cargo run
```

Server will start on `http://localhost:3001`

### Build Release Binary

```bash
cargo build --release
```

Binary will be at `target/release/payment-backend-rust`

### Run Release Binary

```bash
./target/release/payment-backend-rust
```

## 📝 Example Usage

### Using cURL

**Initialize Payment:**

```bash
curl -X POST http://localhost:3001/initialize \
  -H "Content-Type: application/json" \
  -d '{
    "email": "customer@example.com",
    "amount": "50000"
  }'
```

**Health Check:**

```bash
curl http://localhost:3001/health
```

**Redirect Payment:**

```bash
curl -X POST http://localhost:3001/initialize/redirect/customer@example.com/50000
```

## 🔐 Security Features

- **Webhook Signature Verification**: All incoming webhooks are verified using HMAC-SHA512 with your Paystack API key
- **Bearer Token Authentication**: API requests use Bearer token authentication with Paystack
- **Error Handling**: Proper HTTP status codes prevent information leakage

## 📊 Architecture

The application follows a clean structure:

1. **Data Structures** - Serializable request/response models
2. **Handlers** - Async request handlers for each endpoint
3. **Webhook Processing** - Secure webhook verification and processing
4. **Router** - Axum router configuration with all routes
5. **Main** - Server initialization and startup

## 🛠️ Project Structure

```
payment-backend-rust/
├── Cargo.toml           # Project manifest and dependencies
├── Cargo.lock           # Locked dependency versions
├── src/
│   └── main.rs          # Main application code
├── target/              # Build artifacts
└── README.md            # This file
```

## 🐛 Debugging

Enable logging by checking the console output:

```bash
RUST_LOG=debug cargo run
```

The application prints server startup information and webhook events to stdout.

## 📚 Learning Resources

- [Axum Documentation](https://docs.rs/axum)
- [Tokio Documentation](https://tokio.rs)
- [Paystack API Documentation](https://paystack.com/docs)
- [Rust Book](https://doc.rust-lang.org/book)

## 💡 Future Enhancements

- [ ] Database integration for transaction storage
- [ ] Payment history and analytics
- [ ] Advanced error logging and monitoring
- [ ] Rate limiting
- [ ] Transaction status polling
- [ ] Support for multiple payment methods
- [ ] Customer management

## 📄 License

This project is part of my Web3bridge Learning Program.
