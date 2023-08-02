folder_name=$1
shift
command="$@"
absolute_path=$(realpath "$folder_name")
container_id=$(docker run --rm -d -i -t -v "$absolute_path:/app" alpine sh script.sh $command)
#container_id=$(docker run -d -i -t -v $absolute_path:/app alpine)
#out=$(docker exec "$container_id" sh script.sh $command)
code=$(timeout 2 docker wait "$container_id")

#docker logs "$container_id"
docker kill $container_id > /dev/null 2>&1
: '
echo -n 'status: '
if [ -z "$code" ]; then
    echo timeout
else
    echo exited: $code
fi
echo output:
cat $folder_name/completed
'


#sudo docker logs $container_id | sed 's/^/\t/'
#sudo docker rm $container_id &> /dev/null
