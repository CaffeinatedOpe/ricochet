## The Main Point
Configuring links through ConfigMaps was the main reason this project was created. There are two ways to enable it, as described below. The only usage of it currently is to configure redirection paths, and there are no plans to extend this functionality.  
To have a service reboot after the ConfigMap changes, I suggest [Reloader](https://github.com/stakater/Reloader). I use it myself, and it's been working great so far.
## ENV config
To enable the ConfigMap paths via environment variables, just set `CONFIGMAP` to something. I set it to 1, but I believe it can be set to anything.
## CLI config
To enable via the cli, just add the `-C` flag. This is uncommon in kubernetes, so the ENV method is suggested.
## Where to mount
Mount your ConfigMap as a volume to `/config/` in the pod. Other locations are not currently supported, but may be implemented in the future.