kind: PersistentVolumeClaim
apiVersion: v1
metadata:
  name: {name}
  annotations:
    volume.beta.kubernetes.io/storage-class: standard
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: {storage}
