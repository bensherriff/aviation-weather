FROM node:18-alpine AS base

# ============
# Builder Stage
# ============
FROM base AS builder
RUN apk add --no-cache libc6-compat
WORKDIR /builder
COPY . .
RUN \
  if [ -f package.json ]; then  \
    npm i && \
    npm run build; \
  else  \
    echo "package.json not found." && \
    exit 2; \
  fi

# ============
# Runtime Stage
# ============
FROM base AS runner
ARG PORT=3000
ENV PORT=${PORT}
ENV NODE_ENV=production
WORKDIR /app

COPY --chown=node --from=builder /builder /app

USER node

EXPOSE ${PORT}
CMD ["npm", "run", "dev"]
