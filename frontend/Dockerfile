# Build stage
FROM node:20-slim AS builder

WORKDIR /app
COPY package*.json ./
RUN npm ci

COPY . .
RUN npm run build

# Runtime stage
FROM node:20-slim

WORKDIR /app
COPY --from=builder /app/build ./build
COPY --from=builder /app/package*.json ./
RUN npm ci --omit=dev

ENV PUBLIC_API_URL=http://backend:3001/api

EXPOSE 5173

CMD ["node", "build"]
