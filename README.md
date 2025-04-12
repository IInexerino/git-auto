# Simple way of calling 3 git commands at once
ARGUMENTS:

-m "message"
-r "link to repo"      (Optional: "origin" by default)
-b "name of branch"    (Optional: "master" by default)

- It runs [git add .] on your current directory
- Then it commits all of your changes with the specified message
- And then it pushes it to "origin main", unless otherwise specified