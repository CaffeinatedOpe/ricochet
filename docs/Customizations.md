## Customize your service!  
By default, if someone tries to access a path that isn't configured, or tries to access the tld with no path, it will redirect to the github page for the project. This is intentional, but is easy to change.
## Toml:
Should you add the `[behaviors]` section to your toml, you'll be able to configure the `default_page` value to change this default path. The behaviors section currently has no other use.
## CLI:
By adding the `-d "your site here"` cli argument, you can set your default site to whatever is in the quotes.
## ENV:
If you set the `CUSTOM_DEFAULT` variable, the program will redirect any errored pages to the url stored in that variable.