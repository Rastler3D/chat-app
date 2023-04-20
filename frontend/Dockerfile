FROM node:18-alpine AS builder

WORKDIR /usr/src/chat_app

COPY package.json package-lock.json ./

RUN npm install

COPY . .

RUN npm run build

FROM node:18-alpine

WORKDIR /bin/chat_app

COPY --from=builder /usr/src/chat_app/package.json /usr/src/chat_app/build  ./

RUN npm install --omit=dev

EXPOSE 3000

CMD ["node", "index.js"]