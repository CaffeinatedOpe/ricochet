## Welcome to the ricochet wiki!
At the moment, there's not much to see. The project is pretty barebones, and simple to set up. However, some pages are written to make your setup process a bit easier.

## Things to Know
* All links must include the `http://` or `https://` at the beginning, or the path will not redirect properly
* For all aspects, some config methods will overwrite others. Command line args will overwrite environment variables, which will overwrite config files (which obviously overwrite the defaults).
* ConfigMaps and toml configs cannot be used at the same time. Toml is the default, which can be manually overwritten to use configmaps.