apiVersion: apps/v1
kind: StatefulSet
# metadata: <metadata>
spec:
  selector:
    matchLabels:
      app: {name}
  # serviceName:
  replicas: 1
  minReadySeconds: 0
  template:
    metadata:
      labels:
        app: {name}
    spec:
      terminationGracePeriodSeconds: 1
      containers:
      - name: main
        image: <image>
        ports:
        - containerPort: 80
        # volumeMounts:
        # - name: <vol-name>
        #   mountPath: <mount-path>
  # volumeClaimTemplates:
  # - metadata:
  #     name: <metadata>
  #   spec:
  #     accessModes: [ "ReadWriteOnce" ]
  #     storageClassName: "my-storage-class"
  #     resources:
  #       requests:
  #         storage: 1Gi
