# Spotify game app

## Running dev mode

create .env file in the backend folder containing

```bash
SPOTIFY_CLIENT_ID=<your spotify client id>
SPOTIFY_CLIENT_SECRET=<your spotify client secret>
```

create .env file in the frontend folder containing

```bash
PUBLIC_BACKEND_URL=<URI of backend>
```

Open a terminal

```bash
cd backend/
./scripts/setup_js.sh
cargo r
```

Open a second terminal

```bash
cd client/
npm ci
npm run dev
```

## To add

Make issue
