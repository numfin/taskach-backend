FROM node:alpine

WORKDIR /usr/app

COPY package*.json ./
RUN npm ci

ENV NODE_ENV=production
COPY . .
RUN npm run build

CMD [ "node", "dist/index.js" ]