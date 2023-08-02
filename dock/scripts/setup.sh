folder_name=$1
container_id=$(sudo docker run -d -i -t -v ./temp/$folder_name:/app alpine)
out=$(docker exec "$container_id" sh script.sh python3 app.py)
code=$(timeout 2 docker wait "$container_id" || true)

sudo docker kill $container_id &> /dev/null
echo -n 'status: '
if [ -z "$code" ]; then
    echo timeout
else
    echo exited: $code
fi

echo output:
cat temp/$folder_name/completed

#sudo docker logs $container_id | sed 's/^/\t/'
#sudo docker rm $container_id &> /dev/null
