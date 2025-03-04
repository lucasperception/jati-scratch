FROM rust:1-bookworm

RUN apt-get update && apt-get -yqq --no-install-recommends install unzip curl \
    python3 python3-venv git libgomp1 libopenblas-dev build-essential cmake

RUN cd /opt && mkdir raxml-ng && cd raxml-ng \
    && curl -LO https://github.com/amkozlov/raxml-ng/releases/download/1.2.2/raxml-ng_v1.2.2_linux_x86_64.zip \
    && unzip raxml-ng_v1.2.2_linux_x86_64.zip \
    && cd /bin && ln -s /opt/raxml-ng/raxml-ng raxml-ng

RUN cd /opt && mkdir pythia && cd pythia \
    && python3 -m venv venv && . venv/bin/activate \
    && pip install git+https://github.com/tschuelia/PyPythia.git \
    && echo '. /opt/pythia/venv/bin/activate' >> /root/.bashrc

WORKDIR /opt/corax
RUN --mount=type=bind,source=./CMakeLists.fix.txt,target=/tmp/corax-fix git clone https://codeberg.org/Exelixis-Lab/coraxlib.git \
    && cd coraxlib \
    && cat /tmp/corax-fix >> CMakeLists.txt \
    && mkdir -p build && cd build \
    && cmake .. \
    && make && make install
WORKDIR /opt/tools
RUN --mount=type=bind,rw,source=eval_pythia,target=eval_pythia \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=eval_pythia/target \
    cd eval_pythia && cargo build --release && cd .. \
    && cp -p ./eval_pythia/target/release/eval_pythia run-evaluation
COPY ./eval_pythia/predictor.py .

