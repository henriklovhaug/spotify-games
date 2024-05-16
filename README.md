# Spotify games

## Setup

Create a `.env` file with the following

```
SPOTIFY_CLIENT_ID=
SPOTIFY_CLIENT_SECRET=
REDIRECT_URL=http://localhost:4000/callback
```

Run `pnpm i`

Then move the following files

```bash
RUN mkdir assets
RUN mv node_modules/htmx.org/dist/htmx.min.js ./assets/
RUN mv node_modules/uikit/dist/js/uikit.min.js ./assets/
RUN mv node_modules/uikit/dist/js/uikit-icons.min.js ./assets/
```

While developing it would be advised to run `pnpm run watch` and
`systemfs --no-pid -s http::0.0.0.0:4000 -- cargo watch -x run -w assets -w templates`.
It may be useful to add `-w src` as well.

> [!NOTE]
>
> `cargo watch` and `systemfs` is not default and must be installed with
> `cargo install cargo-watch systemfs`

> [!TIP]
>
> The templating macro is set to release mode for faster recompilation, so it's
> advised to run `cargo b` right after cloning the project.

## Contributing

In any html file run the following formatter **in order**

1. prettier
2. djlint
