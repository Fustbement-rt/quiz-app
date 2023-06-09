FROM rust:1.69 as builder
COPY . .
RUN cargo build --release && strip target/release/quiz-rest-v1

FROM alpine:3.17 as runtime
COPY --from=builder target/release/quiz-rest-v1 .
EXPOSE 3000
CMD ["quiz-rest-v1"]