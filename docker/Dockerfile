# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
  FROM alpine:latest
  WORKDIR /home/myapp/
  RUN mkdir data
  RUN mkdir conf
  COPY app .
  CMD ["./app"]