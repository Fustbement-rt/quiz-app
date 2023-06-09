FROM node:19-alpine AS base
COPY . .
RUN pnpm i && pnpm run build

FROM base AS runtime
ENV NODE_ENV=production
COPY --from=base build .
RUN pnpm i
EXPOSE 3000
STOPSIGNAL SIGINT
CMD ["node", "build/index.js"]
