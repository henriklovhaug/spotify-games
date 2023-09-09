FROM node:alpine as builder

WORKDIR /app

COPY ./package.json ./

RUN npm install

COPY ./ ./

ARG PUBLIC_BACKEND_URL
ENV PUBLIC_BACKEND_URL=${PUBLIC_BACKEND_URL}
RUN printenv PUBLIC_BACKEND_URL > .env

RUN npm run build

FROM denoland/deno:alpine

WORKDIR /app

COPY --from=builder /app/build ./build

CMD [ "run", "--allow-read", "--allow-net", "--allow-env", "/app/build/index.js" ]
