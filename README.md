# Rust Book App 🚀

An example for writing a CRUD app using `warp` and `tokio-postgres`.

Inspired by [this app](https://github.com/zupzup/warp-postgres-example).

## 🏁 A simple API using Rust + PostgreSQL

This is a fictional project for laboratory study written in the **Rust** :crab: programming language.

The project is an **API** with **CRUD** functionalities that uses a PostgreSQL database.

The data is stored in the public db schema in the `book` table.

### 1. 💡 Prerequisites

- [Docker](https://www.docker.com/products/docker-desktop/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Curl](https://curl.se/)
- [Postman](https://www.postman.com/) or [Insomnia](https://insomnia.rest/download) - (_both are optional_)
- [Tableplus](https://tableplus.com/) or [DBeaver](https://dbeaver.io/) - (_both are optional_)

### 2. 🏃 Running the database Docker

For the application to run, a PostgreSQL database is needed.

Deploy a PostgreSQL instance locally with Docker:

```
docker run -it -d -p 5432:5432 -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=postgres --name postgres postgres:latest
```

### 3. 🏗️ Build project

:radioactive: You shall already have **Rust** and **ToolChain** installed on your workstation. :radioactive:

Enter the base directory of the project and run the command:

```bash
cargo build --release
```

This will build the app in **release** mode. After running the project:

```bash
cargo run
```

### Login

```bash
curl http://localhost:8000/login -d '{"email": "user@userland.com", "pw": "1234"}' -H 'Content-Type: application/json'

{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxIiwicm9sZSI6IlVzZXIiLCJleHAiOjE2MDMxMzQwODl9.dWnt5vfcGdwypEQUr3bLMrZYfdyxj3v6-io6VREWHXebMUCKBddf9xGcz4vHrCXruzx42zrS3Kygiqw3xV8W-A"}
```

### 4. 🧪 Testing the CRUD Endpoints

To test the endpoints, you can use **Postman**: the collections are in the [Rust Book API](Rust Book API.postman_collection.json) file.
Alternatively, you can test the API via command line with **curl**.

| Method   | EndPoint | Parameter      | Payload                                                  |
| -------- | -------- | -------------- | -------------------------------------------------------- |
| `POST`   | `/book`  | _not required_ | `{"title": "Test title", "author": "Test author"}`       |
| `GET`    | `/book/` | ID             | _not required_                                           |
| `PUT`    | `/book/` | ID             | `{"title": "Another title", "author": "Another author"}` |
| `GET`    | `/book`  | _not required_ | _not required_                                           |
| `DELETE` | `/book/` | ID             | _not required_                                           |

> Note: The commands below use `curl`.

#### 4.1 📝 Creating a user

Command:

```bash
curl -i -H "Content-Type: application/json" -H 'Authorization: Bearer [JWT_TOKEN_HERE]' -X POST http://127.0.0.1:8080/book -d '{"title": "Test title", "author": "Test author"}'
curl -i -H "Content-Type: application/json" -H 'Authorization: Bearer [JWT_TOKEN_HERE]' -X POST http://127.0.0.1:8080/book -d '{"title": "Another title", "author": "Another author"}'
```

#### 4.2 📝 Checking created user with ID

Command:

```bash
curl -i -H "Content-Type: application/json" -H 'Authorization: Bearer [JWT_TOKEN_HERE]' -X GET http://127.0.0.1:8080/book/1
```

#### 4.3 📝 Updating user data

Command:

```bash
curl -i -H "Content-Type: application/json" -H 'Authorization: Bearer [JWT_TOKEN_HERE]' -X PUT http://127.0.0.1:8080/book/1 -d '{"title": "Another title", "author": "Another author"}'
```

#### 4.4 📝 Checking all registered book

Command:

```bash
curl -i -H "Content-Type: application/json" -H 'Authorization: Bearer [JWT_TOKEN_HERE]' -X GET http://127.0.0.1:8080/book
```

Expected answer:

#### 4.5 📝 Deleting a user with ID

Command:

```bash
curl -i -H "Content-Type: application/json" -H 'Authorization: Bearer [JWT_TOKEN_HERE]' -X DELETE http://127.0.0.1:8080/book/1
curl -i -H "Content-Type: application/json" -H 'Authorization: Bearer [JWT_TOKEN_HERE]' -X DELETE http://127.0.0.1:8080/book/2
```

**Enjoy!** :tropical_drink:
