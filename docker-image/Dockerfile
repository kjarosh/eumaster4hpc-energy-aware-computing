FROM fedora:38

RUN dnf install -y python3-notebook \
  	&& dnf clean all \
  	&& rm -rf /var/cache/yum

COPY jupyter /jupyter
WORKDIR /jupyter

ENTRYPOINT [ \
    "/usr/bin/jupyter-notebook", \
    "--allow-root", \
    "--port", "8888", \
    "--ip", "0.0.0.0"]
