docker stop awablockdev
docker rm awablockdev
docker image rm awablock
docker build -f ./envs/dev/Dockerfile -t awablock .
docker run -d --name awablockdev -p 8080:8080 awablock
