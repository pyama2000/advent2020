FROM node:12 as builder
WORKDIR /usr/src/musa_client
COPY package.* ./
RUN npm install
COPY . .
RUN npm run build

FROM node:12-alpine
ENV NODE_ENV=production
ENV HOST=0.0.0.0
EXPOSE 3000
WORKDIR /musa_client
COPY --from=builder /usr/src/musa_client/package.json package.json
COPY --from=builder /usr/src/musa_client/nuxt.config.js nuxt.config.js
COPY --from=builder /usr/src/musa_client/node_modules node_modules/
COPY --from=builder /usr/src/musa_client/.nuxt .nuxt/
COPY --from=builder /usr/src/musa_client/static static/
CMD ["yarn", "start"]
