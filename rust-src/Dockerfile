# Compile
FROM registry.cn-hangzhou.aliyuncs.com/gebilaoyu/rust:1.79.0 AS compiler


WORKDIR /lottery-rs
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
ADD . /api /lottery-rs/api/
ADD . /common /lottery-rs/common/
ADD . /entity /lottery-rs/entity/
ADD . /migration /lottery-rs/migration/
ADD . /model /lottery-rs/model/
ADD . /security /lottery-rs/security/
ADD . /service /lottery-rs/service/
ADD . /src /lottery-rs/src/
COPY . /config.yaml /lottery-rs/
COPY . /Cargo.toml /lottery-rs/
RUN cargo build --release


# Run
FROM registry.cn-hangzhou.aliyuncs.com/gebilaoyu/ubuntu:24.10

COPY --from=compiler /lottery-rs/target/release/lottery-rs /apps/
COPY --from=compiler /lottery-rs/config.yaml /apps/

#ENV HOST=0.0.0.0
#ENV PORT=8080
#ENV RUST_LOG=info
#ENV DATABASE_URL=sqlite://db.sqlite?mode=rwc

WORKDIR /apps
CMD ["./lottery-rs"]
