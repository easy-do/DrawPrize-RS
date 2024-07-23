# Compile
FROM registry.cn-hangzhou.aliyuncs.com/gebilaoyu/rust:1.79.0 AS compiler


WORKDIR /DrawPrize-RS
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
ADD . /api /DrawPrize-RS/api/
ADD . /common /DrawPrize-RS/common/
ADD . /entity /DrawPrize-RS/entity/
ADD . /migration /DrawPrize-RS/migration/
ADD . /model /DrawPrize-RS/model/
ADD . /security /DrawPrize-RS/security/
ADD . /service /DrawPrize-RS/service/
ADD . /src /DrawPrize-RS/src/
COPY . /config.yaml /DrawPrize-RS/
COPY . /Cargo.toml /DrawPrize-RS/
ADD . /static /DrawPrize-RS/static/
RUN cargo build --release


# Run
FROM registry.cn-hangzhou.aliyuncs.com/gebilaoyu/ubuntu:24.10

COPY --from=compiler /DrawPrize-RS/target/release/DrawPrize-RS /apps/
COPY --from=compiler /DrawPrize-RS/config.yaml /apps/
COPY --from=compiler /DrawPrize-RS/static /apps/static/

#ENV HOST=0.0.0.0
#ENV PORT=8080
#ENV RUST_LOG=info
#ENV DATABASE_URL=sqlite://db.sqlite?mode=rwc

WORKDIR /apps
CMD ["./DrawPrize-RS"]
