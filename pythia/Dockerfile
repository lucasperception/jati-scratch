FROM rust:1-bookworm

RUN apt-get update && apt-get -yqq --no-install-recommends install unzip curl \
    python3 python3-venv git libgomp1 libopenblas-dev build-essential cmake libclang-dev

WORKDIR /opt/raxml-ng
RUN curl -LO https://github.com/amkozlov/raxml-ng/releases/download/1.2.2/raxml-ng_v1.2.2_linux_x86_64.zip \
    && unzip raxml-ng_v1.2.2_linux_x86_64.zip \
    && cd /bin && ln -s /opt/raxml-ng/raxml-ng raxml-ng

WORKDIR /opt/pythia
RUN python3 -m venv venv && . venv/bin/activate \
    && pip install git+https://github.com/tschuelia/PyPythia.git \
    && echo '. /opt/pythia/venv/bin/activate' >> /root/.bashrc

WORKDIR /opt/tools
RUN --mount=type=bind,rw,source=eval_pythia,target=eval_pythia cd eval_pythia && cargo build --release && cd .. \
    && cp -p ./eval_pythia/target/release/runner run-evaluation && cp -rp ./eval_pythia/target/release/build build-out
COPY ./eval_pythia/predictor.py .

