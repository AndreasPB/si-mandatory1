FROM node:16-alpine

WORKDIR /usr/src/app

COPY . .

RUN yarn install && yarn cache clean

EXPOSE 3000
EXPOSE 24678

CMD yarn run dev
