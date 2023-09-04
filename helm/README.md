# Helm

## Usage

```bash
cd AppChart/

# run app
$ helm install flaskapp .

# with custom values
$ helm install -f values.deploy.yaml flaskapp .

# delete app
$ helm uninstall flaskapp
```
