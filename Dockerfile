FROM debian:trixie-slim AS dict-extract-env

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN apt-get update && \
	apt-get install -y wget xz-utils zstd

RUN wget https://github.com/daac-tools/vibrato/releases/download/v0.5.0/unidic-cwj-3_1_1.tar.xz && \
	tar xf unidic-cwj-3_1_1.tar.xz && \
	cd unidic-cwj-3_1_1 && \
	unzstd system.dic.zst && \
	rm system.dic.zst && \
	cd .. && \
	mkdir /usr/share/lingua/ && \
	mv unidic-cwj-3_1_1 /usr/share/lingua/

FROM rust:1.92.0 AS build-env
LABEL maintainer="yanorei32"

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

WORKDIR /usr/src

RUN cargo new g2p-server

COPY LICENSE Cargo.toml Cargo.lock /usr/src/g2p-server/

WORKDIR /usr/src/g2p-server

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

RUN	cargo install cargo-license && cargo license \
	--authors \
	--do-not-bundle \
	--avoid-dev-deps \
	--avoid-build-deps \
	--filter-platform "$(rustc -vV | sed -n 's|host: ||p')" \
	> CREDITS

RUN cargo build --release
COPY src/ /usr/src/g2p-server/src/
COPY assets/ /usr/src/g2p-server/assets/
RUN touch src/* && cargo build --release

FROM debian:trixie-slim

WORKDIR /

COPY --chown=root:root --from=dict-extract-env \
	/usr/share/lingua \
	/usr/share/lingua

COPY --chown=root:root --from=build-env \
	/usr/src/g2p-server/CREDITS \
	/usr/src/g2p-server/LICENSE \
	/usr/share/licenses/g2p-server/

COPY --chown=root:root --from=build-env \
	/usr/src/g2p-server/target/release/g2p-server \
	/usr/bin/g2p-server

CMD ["/usr/bin/g2p-server"]
