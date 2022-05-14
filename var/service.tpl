apiVersion: v1
kind: Service
metadata:
  name: {name}
  labels:
    app: {name}
spec:
  ports:
  - port: {port}
    # name: <port-name>
  clusterIP: None
  selector:
    app: {name}
