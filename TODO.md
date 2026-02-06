# Main Goals
* ~~redirect incoming traffic to external site based on path using data in a hashmap~~
* ~~pull data from config file~~
* build and deploy with CI/CD loop
* core configs through env. variables or cli args
* pull data from ConfigMap
* Readme/documentation
* example deployments

# stretch goals:
* integrate tests
* check if hashmaps are more optimized than searching a pair of vectors. doubt it, but possible optimization opportunity
* find better way to pass through/find mapping?
* random/round-robin redirection
* rewrite to not need actix/actix web? (probably not)