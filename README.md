# tiny-ip

```bash
podman machine init
podman machine start
podman build -t rosvik/tiny-ip:v0.1 .
podman run -p 8000:8000 -d rosvik/tiny-ip:v0.1
podman ps
```

### logs

```bash
podman logs -f <container-id>
```

### Attach with ssh

```bash
podman exec -it <container-id> /bin/bash
```

