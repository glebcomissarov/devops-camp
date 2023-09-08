# Helm

## Usage

```bash
cd AppChart/

# run app
$ helm install superApp .

# with custom values
$ helm install -f values.deploy.yaml superApp .

# delete app
$ helm uninstall superApp
```
