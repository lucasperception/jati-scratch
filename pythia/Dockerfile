FROM debian:bookworm

RUN apt-get update && apt-get -yqq --no-install-recommends install unzip curl python3 python3-venv git libgomp1

RUN cd /opt && mkdir raxml-ng && cd raxml-ng \
    && curl -LO https://github.com/amkozlov/raxml-ng/releases/download/1.2.2/raxml-ng_v1.2.2_linux_x86_64.zip \
    && unzip raxml-ng_v1.2.2_linux_x86_64.zip \
    && cd /bin && ln -s /opt/raxml-ng/raxml-ng raxml-ng

RUN cd /opt && mkdir pythia && cd pythia \
    && python3 -m venv venv && . venv/bin/activate \
    && pip install git+https://github.com/tschuelia/PyPythia.git \
    && echo '. /opt/pythia/venv/bin/activate' >> /root/.bashrc
