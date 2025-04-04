FROM node:20.16-slim

COPY ./ ./

RUN npm install

ENV NODE_ENV production
ENV DATABASE postgres

EXPOSE 8080

CMD ["node", "src/clustered.mjs"]
