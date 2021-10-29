# This is a simple actix web app example showing how adom could be used in a project

## Setup

1. get a version of postgres up and running.
2. update the connection string in ./src/config.rs (alternatively you could just set the ENV Variable )
3. make sure the DB in the connection string existest `CREATE DATABASE BLA;`
4. migrate the database to create the orders table and mock data

```
cargo run --bin migrate
```

5. boot the server

```
cargo run --bin server
```

6. You can use curl to see the server in action

```
curl http://localhost:5000/orders
```
