# Compile
FROM registry.cn-hangzhou.aliyuncs.com/gebilaoyu/rust:1.79.0 AS compiler


WORKDIR /prize-rs
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
ADD . /api /prize-rs/api/
ADD . /common /prize-rs/common/
ADD . /entity /prize-rs/entity/
ADD . /migration /prize-rs/migration/
ADD . /model /prize-rs/model/
ADD . /security /prize-rs/security/
ADD . /service /prize-rs/service/
ADD . /src /prize-rs/src/
COPY . /config.yaml /prize-rs/
COPY . /Cargo.toml /prize-rs/
RUN cargo build --release


# Run
FROM registry.cn-hangzhou.aliyuncs.com/gebilaoyu/ubuntu:24.10

COPY --from=compiler /prize-rs/target/release/prize-rs /apps/
COPY --from=compiler /prize-rs/config.yaml /apps/

#ENV HOST=0.0.0.0
#ENV PORT=8080
#ENV RUST_LOG=info
#ENV DATABASE_URL=sqlite://db.sqlite?mode=rwc

WORKDIR /apps
CMD ["./prize-rs"]
