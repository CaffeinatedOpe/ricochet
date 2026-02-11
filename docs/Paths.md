## Your Paths
To get up and running, you've got to get some paths configured. This can either be done with a configmap or the toml config file.
## Toml
In your configuration toml, you'll need a `[paths]` header. This is required, and your instances will crash without it. The syntax for paths is `subdirectory = 'target'`, where "subdirectory" is replaced by the path following the slash in your url, and "target" is the page you want
the path to redirect to.  

Example:
```
[paths]
blog = 'https://caffeinatedope.net/blog'
github = 'https://github.com/CaffeinatedOpe/ricochet'
```
Behavior, assuming base url is ricochet.caffeinatedope.net:  
- going to `ricochet.caffeinatedope.net/blog` leads to `https://caffeinatedope.net/blog`
- going to `ricochet.caffeinatedope.net/github` leads to `https://github.com/CaffeinatedOpe/ricochet`  

## ConfigMap
If you decide to go with ConfigMaps via kubernetes... you are the reason this project exists.  
Getting going is just as simple. You'll need either the `-C` flag set, or the `CONFIGMAP` env variable needs to be set, so the program knows to read from the directory as a ConfigMap. I believe the env variable can be set to anything, but I haven't tested that. To define your paths in the map by setting the subpath as a key value, and the target as the value.

Example:  
```
kind: ConfigMap
apiVersion: v1
metadata:
  name: ricochet-configmap
  namespace: ricochet
data:
  creator: https://caffeinatedope.net/
  github: https://github.com/CaffeinatedOpe/ricochet
  awesome: https://caffeinatedope.net/awesome
```
A full kubernetes deployment example (minus ingress) can be found in the examples folder of the repo.