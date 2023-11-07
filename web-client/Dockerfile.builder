# The base builder will have trunk installed
FROM us-central1-docker.pkg.dev/hubs-dev-333333/ocho-osai/johnshaughnessy/track/track-client-base-builder as builder
WORKDIR /track
COPY . .
RUN ls -la
RUN ls -la web-client/
RUN trunk build web-client/index.html # Build once so that dependencies are cached
RUN rm -rf web-client/dist # The built files are not needed in the final image

# Upload this image as track-client-builder
