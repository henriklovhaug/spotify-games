# Spotify games

## Setup

Create a `.env` file with the following

```
SPOTIFY_CLIENT_ID=
SPOTIFY_CLIENT_SECRET=
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
`cargo watch -x run -w assets -w templates`. It may be useful to add `-w src` as
well.

> [!NOTE]
>
> `cargo watch` is not default and must be installed with
> `cargo install cargo-watch`

> [!TIP]
>
> The templating macro is set to release mode for faster recompilation, so it's
> advised to run `cargo b` right after cloning the project.
