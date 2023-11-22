FROM fedora:38

RUN \
    # install Jupyter
    dnf install -y python3-notebook && \
    # install NVIDIA
    dnf install -y https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm && \
    dnf install -y akmod-nvidia xorg-x11-drv-nvidia-cuda && \
    # clean up
    dnf clean all && \
    rm -rf /var/cache/yum

COPY jupyter /jupyter
WORKDIR /jupyter

ENTRYPOINT [ \
    "/usr/bin/jupyter-notebook", \
    "--allow-root", \
    "--port", "8888", \
    "--ip", "0.0.0.0"]
