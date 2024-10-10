# Emotion Graph


### Setup
1. Clone repo
    ```sh
    $ git clone git@github.com:qweme32/emotion-graph.git && cd emotion-graph
    ```
3. Set secret key ( 16 random chars )
    ```sh
    $ vim ./api/.env
    ```
2.  ```sh
    $ docker-compose up -d
    ```
3. Create first user
    ```sh
    $ curl -X POST http://localhost:8080/admin/new-user?secret=SECRET_KEY_FROM_API_ENV \
    -H "Content-Type: application/json" \
    -d '{"username": "REPLACE 3-12 CHARS", "password": "REPLACE 8-32 CHARS"}'
    ```
4. Open ui `http://localhost:3000`
    