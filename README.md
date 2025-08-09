# Todo API

A RESTful API built with Rust, Poem, and SQLx for managing todos and users.

## Tech Stack

- **Framework**: Poem with OpenAPI support
- **Database**: PostgreSQL with SQLx
- **Runtime**: Tokio

## Prerequisites

- Rust 1.70+
- PostgreSQL
- `.env` file with `DATABASE_URL`

## Setup

1. Create a `.env` file:
```
DATABASE_URL=postgres://user:password@localhost/todo_db
```

2. Run migrations:
```bash
sqlx migrate run
```

3. Build and run:
```bash
cargo run
```

The server will start on `http://localhost:3000`

## API Endpoints

### Users
- `GET /api/v1/users` - List all users

### Todos
- `GET /api/v1/todos` - List all todos
- `POST /api/v1/todos` - Create a new todo
- `GET /api/v1/todos/:id` - Get a specific todo
- `POST /api/v1/todo/:id/complete-todo` - Mark todo as completed

### Documentation
- `GET /docs` - Swagger UI
- `GET /api/v1/spec` - OpenAPI specification

## Development

```bash
# Check code
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Database Schema

### Users
- `id` - Primary key
- `user_name` - Username
- `email` - Email address
- `created_at` - Timestamp

### Todos
- `id` - Primary key
- `user_id` - Foreign key to users
- `title` - Todo title
- `completed` - Completion status
- `created_at` - Timestamp