# Helm

## Usage

```bash
cd AppChart/

# run app
$ helm install super-app .

# with custom values
$ helm install -f values.deploy.yaml super-app .

# delete app
$ helm uninstall super-app
```
