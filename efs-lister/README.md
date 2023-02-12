## work with EFS

cargo lambda invoke --remote \
                --data-ascii '{"name": "Marco"}' \
                --output-format json \
                efs-lister
{
  "files": "\n\n/mnt/efs/foo.txt",
  "msg": "Hello, Marco!",
  "req_id": "4c1e79df-a304-4f97-85b3-ba39e00e04a5"
}