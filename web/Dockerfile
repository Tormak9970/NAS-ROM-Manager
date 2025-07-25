# use the official Bun image
# see all versions at https://hub.docker.com/r/oven/bun/tags
FROM oven/bun:alpine AS base
WORKDIR /usr/src/app

# install dependencies into temp directory
# this will cache them and speed up future builds
FROM base AS install
RUN mkdir -p /temp/dev
COPY package.json bun.lockb /temp/dev/
RUN cd /temp/dev && bun install --frozen-lockfile

# install with --production (exclude devDependencies)
RUN mkdir -p /temp/prod
COPY package.json bun.lockb /temp/prod/
RUN cd /temp/prod && bun install --frozen-lockfile --production

# copy node_modules from temp directory
# then copy all (non-ignored) project files into the image
FROM base AS prerelease
COPY --from=install /temp/dev/node_modules node_modules
COPY . .

ENV NODE_ENV=production

# [optional] tests & build
RUN bun run "build:svelte"

# copy production dependencies and source code into final image
FROM base AS release
COPY --from=install /temp/dev/node_modules node_modules
COPY --from=prerelease /usr/src/app/build .
COPY --from=prerelease /usr/src/app/package.json .

RUN chmod -R 777 /usr/src/app/client
RUN chmod -R 777 /usr/src/app/server

COPY --from=scripts ./start.sh /docker-entrypoint.d/start.sh
RUN dos2unix /docker-entrypoint.d/start.sh
RUN chmod +x /docker-entrypoint.d/start.sh

# run the app
USER bun
EXPOSE 3000/tcp
ENTRYPOINT [ "/docker-entrypoint.d/start.sh" ]