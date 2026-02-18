# Email Newsletter Service

## Demo
https://github.com/user-attachments/assets/8a7ce117-1f75-4103-a28d-5be563fa43fc

## Running it locally
#### Prerequisites:
- Docker
OR
- PostgreSQL (optional)
- Redis (optional)
- sqlx cli (https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
- Rust and Cargo 
#### Automatic setup with Docker Compose:
1. Clone the repository:
   ```bash
   git clone https://github.com/arjav0703/email-newsletter.git
    cd email-newsletter
    ```

2. Setup Config file (see example.config.toml for reference):
    ```bash
    cp example.config.toml config.yaml
    ```
3. Run the application with Docker Compose:
    ```bash
    docker-compose up -d --build
    ```
    
#### Manual setup:
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


### Acknowledgement
This project was built while following the book "[Zero To Production In Rust](https://www.zero2prod.com/)" by Luca Palmieri. Though the core architecture of the project was inspired by the book, I tried to challenge myselves to do the implementation on my own as much as possible. I also additional features and improvements to the original project (like a fully functional frontend, Docker support and other code changes). Also, i chose a different way to send emails (using `resend` instead of `Postmark` service).

### AI usage Disclosure
I had tab completion turned on and it helped me to develop faster. I did not use any specific AI IDEs (like cursor, claude code) and sticked to neovim for development. The only area where I used AI relatively more than other parts is debugging the authentication test logic. 

