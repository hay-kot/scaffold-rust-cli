# GoReleaser builds static musl binaries outside Docker and places them at
# <os>/<arch>/{{ .ProjectKebab }} in the build context.
FROM gcr.io/distroless/static-debian12:nonroot
ARG TARGETPLATFORM
COPY ${TARGETPLATFORM}/{{ .ProjectKebab }} /usr/local/bin/{{ .ProjectKebab }}
ENTRYPOINT ["/usr/local/bin/{{ .ProjectKebab }}"]
