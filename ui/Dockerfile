FROM node:18-alpine AS base

FROM base AS builder
RUN apk add --no-cache libc6-compat
WORKDIR /builder
COPY . .
RUN \
  if [ -f package.json ]; then npm i && npm run build; \
  else echo "package.json not found." && exit 2; \
  fi

FROM base AS runner
ARG PORT=3000
ENV PORT=${PORT}

WORKDIR /app

ENV NODE_ENV=production

#RUN addgroup --system --gid 1001 node
#RUN adduser --system --uid 1001 node

COPY --from=builder /builder /app
#RUN chown -R node:node /app

USER node

EXPOSE ${PORT}

CMD ["npm", "run", "dev"]
