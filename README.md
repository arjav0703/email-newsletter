# Email Newsletter Service

## Running it locally
#### Prerequisites:
- Docker
OR
- PostgreSQL (optional)
- Redis (optional)
- sqlx cli (https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
- Rust and Cargo 

#### Steps:
1. Clone the repository:
   ```bash
   git clone https://github.com/arjav0703/email-newsletter.git
    cd email-newsletter
    ```

2. Setup Config file (see example.config.toml for reference):
   ```bash
   cp example.config.toml config.yaml
   ```

3. Setup PostgreSQL database:
    - Using docker (automatic):
        ```bash
        scripts/postgres.sh
        ```
    - Using local PostgreSQL (manual):
        - Create a database named `newsletter`
        - Run migrations using sqlx:
        ```bash
        sqlx migrate run
        ```
        - Update the `config.yaml` file with your database credentials

4. Setup Redis:
    - Using docker (automatic):
        ```bash
        scripts/redis.sh
        ```
    - Using local Redis (manual):
        - Install Redis and start the server
        - Update the `config.yaml` file with your Redis credentials

5. Run the application:
    - With Cargo:
   ```bash
   cargo run 
   ```
    - With Docker:
   ```bash
    docker build -t email-newsletter .
    docker run -p 8000:8000 email-newsletter
    ```

6. Visit the URL `http://localhost:8000` (or the one you setup) to access the application.

